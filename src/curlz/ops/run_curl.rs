use crate::data::HttpRequest;
use crate::ops::{MutOperation, OperationContext};
use crate::Result;

use anyhow::Context;
use liquid::Parser;
use liquid_core::Object;
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

    fn execute(self, context: &mut OperationContext) -> crate::Result<Self::Output> {
        let env = context.environment_mut();

        // todo: here the placeholders needs to be merged with the environment
        self.request.placeholders.iter().for_each(|placeholder| {
            // todo: no unwrapping here
            let value = placeholder
                .value
                .as_ref()
                .unwrap_or_else(|| placeholder.default.as_ref().unwrap());
            env.insert(&placeholder.name, value);
        });

        let engine = liquid::ParserBuilder::with_stdlib().build()?;
        let ctx: Object = env.into();

        let url_template = engine.parse(self.request.url.as_str())?;
        let url = url_template.render(&ctx)?;

        let method: String = (&self.request.method).into();

        let mut cmd = Command::new("curl");
        cmd.args(&["-X", &method])
            .args(
                &self
                    .request
                    .curl_params
                    .iter()
                    .map(|s| parse(&engine, &ctx, s))
                    .collect::<Result<Vec<_>>>()?,
            )
            .args(self.request.headers.as_ref().iter().flat_map(|(k, v)| {
                vec![
                    "-H".to_string(),
                    parse(&engine, &ctx, format!("{}: {}", k, v)).unwrap(),
                ]
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

fn parse(parser: &Parser, ctx: &Object, str: impl AsRef<str>) -> Result<String> {
    parser
        .parse(str.as_ref())
        .and_then(|s| s.render(ctx))
        .map_err(|e| e.into())
}
