use crate::data::HttpRequest;
use crate::ops::{MutOperation, OperationContext, Verbosity};
use crate::Result;

use anyhow::Context;
use std::process::{Command, Stdio};

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

    fn execute(&self, context: &mut OperationContext) -> crate::Result<Self::Output> {
        let mut renderer = context.renderer_with_placeholders(&self.request.placeholders);

        let url = renderer.render(self.request.url.as_ref(), "url")?;
        let method: String = (&self.request.method).into();

        let mut cmd = Command::new("curl");
        if context.verbosity == Verbosity::None {
            cmd.arg("-s");
        }
        cmd.args(&["-X", &method])
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
            .arg(&url);

        // todo: only display this in verbose mode
        // println!("{:?}", &cmd);

        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map(|_output| ())
            .context("error when starting curl")
    }
}
