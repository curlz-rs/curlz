use crate::data::HttpRequest;
use crate::ops::{MutOperation, OperationContext};
use crate::Result;

use anyhow::Context;
use std::process::{Command, Stdio};
use minijinja::value::Value;

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

        let ctx: minijinja::value::Value = env.into();
        let mut env = minijinja::Environment::new();
        let url = parse(&mut env, &ctx, self.request.url.as_ref(), "url")?;
        let method: String = (&self.request.method).into();

        let mut cmd = Command::new("curl");
        cmd.args(&["-X", &method])
            .args(
                &self
                    .request
                    .curl_params
                    .iter()
                    .map(|s| parse(&mut env, &ctx, s, "param"))
                    .collect::<Result<Vec<_>>>()?,
            )
            .args(self.request.headers.as_ref().iter().flat_map(|(k, v) |{
                let value = parse(&mut env, &ctx, v, k).unwrap();
                vec![
                    "-H".to_string(),
                    format!("{}: {}", k, value)
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

fn parse<'source>(env: &mut minijinja::Environment<'source>, ctx: &Value, str: &'source str, name: &'source str) -> Result<String> {
    env.add_template(name, str)?;
    let template = env.get_template(name)?;

    template.render(&ctx).map_err(|e| e.into())
}
