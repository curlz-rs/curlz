use crate::data::{HttpBody, HttpRequest};
use crate::ops::{MutOperation, OperationContext, Verbosity};
use crate::Result;
use std::ffi::OsStr;

use anyhow::Context;
use log::debug;
use std::process::{Command, Stdio};

/// consumes self, and turns it into arguments for a curl [`Command`]
pub trait IntoCurlArguments<I, S> {
    fn as_curl_parameter(&self) -> I
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
}

#[derive(Debug)]
pub struct RunCurlCommand<'a> {
    pub request: &'a HttpRequest,
}

impl<'a> RunCurlCommand<'a> {
    pub fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> MutOperation for RunCurlCommand<'a> {
    type Output = ();

    fn execute(&self, context: &mut OperationContext) -> Result<Self::Output> {
        let mut renderer = context.renderer_with_placeholders(&self.request.placeholders);

        let url = renderer.render(self.request.url.as_ref(), "url")?;
        let _method: String = (&self.request.method).into();

        let mut cmd = Command::new("curl");
        if context.verbosity.eq(&Verbosity::Silent) {
            cmd.arg("-s");
        }
        let payload = if self.request.body.ne(&HttpBody::None) {
            vec![
                "--data".to_string(),
                match &self.request.body {
                    HttpBody::InlineText(s) => renderer.render(s.as_str(), "body")?,
                    HttpBody::InlineBinary(_) => todo!("inline binary data not impl yet"),
                    HttpBody::Extern(_) => todo!("external file data loading impl yet"),
                    HttpBody::None => "".to_string(),
                },
            ]
        } else {
            vec![]
        };

        cmd.args(self.request.method.as_curl_parameter())
            .args(
                &self
                    .request
                    .curl_params
                    .iter()
                    .map(|s| renderer.render(s, "param"))
                    .collect::<Result<Vec<_>>>()?,
            )
            .args(self.request.headers.as_ref().iter().flat_map(|(k, v)| {
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
    use super::IntoCurlArguments;
    use crate::data::HttpMethod;

    /// todo: not yet sure if that abstraction is really helpful or stands in the way
    impl IntoCurlArguments<Vec<String>, String> for HttpMethod {
        fn as_curl_parameter(&self) -> Vec<String> {
            let method: String = self.into();
            vec!["-X".to_string(), method]
        }
    }
}
