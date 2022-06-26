use crate::{CommandContext, Execute, HttpRequest, Result};

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

impl<'a> Execute for RunCurlCommand<'a> {
    type Output = ();

    fn execute(self, context: &CommandContext) -> crate::Result<Self::Output> {
        let env = context.environment();
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
        println!("{:?}", &cmd);

        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
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
