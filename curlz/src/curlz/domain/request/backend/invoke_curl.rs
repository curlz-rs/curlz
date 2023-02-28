use self::curl_arg_conversions::IntoCurlArguments;
use super::HttpBackend;
use crate::domain::http::HttpBody;
use crate::domain::request::{IssueRequest, RequestContext, Verbosity};
use crate::Result;

use anyhow::Context;
use log::debug;
use std::process::{Command, Stdio};

#[derive(Default)]
pub struct InvokeCurlBackend;

/// It knows haw to issue a `HttpRequest`
impl HttpBackend for InvokeCurlBackend {
    fn issue(&self, req: &IssueRequest, context: &RequestContext) -> Result<()> {
        let request = req.request;
        let mut renderer = context.renderer_with_placeholders(&request.placeholders);

        let url = renderer.render(request.url.as_ref(), "url")?;
        let _method: String = (&request.method).into();

        let mut cmd = Command::new("curl");
        if req.verbosity.eq(&Verbosity::Silent) {
            cmd.arg("-s");
        }
        let payload = if request.body.ne(&HttpBody::None) {
            vec![
                "--data".to_string(),
                match &request.body {
                    HttpBody::InlineText(s) => renderer.render(s.as_str(), "body")?,
                    HttpBody::InlineBinary(_) => todo!("inline binary data not impl yet"),
                    HttpBody::Extern(_) => todo!("external file data loading impl yet"),
                    HttpBody::None => "".to_string(),
                },
            ]
        } else {
            vec![]
        };

        cmd.args(request.method.as_curl_parameter())
            .args(
                &request
                    .curl_params
                    .iter()
                    .map(|s| renderer.render(s, "param"))
                    .collect::<Result<Vec<_>>>()?,
            )
            .args(request.headers.as_ref().iter().flat_map(|(k, v)| {
                let value = renderer.render(v, k).unwrap();
                vec!["-H".to_string(), format!("{}: {}", k, value)]
            }))
            .arg(&url)
            .args(&payload);

        debug!("curl cmd: \n  {:?}", &cmd);

        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map(|_output| ())
            .context("error when starting curl")
    }
}

mod curl_arg_conversions {
    use crate::domain::http::HttpMethod;
    use std::ffi::OsStr;

    /// consumes self, and turns it into arguments for a curl [`Command`]
    ///
    pub trait IntoCurlArguments<I, S> {
        fn as_curl_parameter(&self) -> I
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>;
    }

    /// todo: not yet sure if that abstraction is really helpful or stands in the way
    impl IntoCurlArguments<Vec<String>, String> for HttpMethod {
        fn as_curl_parameter(&self) -> Vec<String> {
            let method: String = self.into();
            vec!["-X".to_string(), method]
        }
    }
}
