use crate::data::HttpRequest;
use crate::ops::{Operation, OperationContext, Verbosity};
use crate::Result;

use crate::template::render;
use anyhow::Context;
use minijinja::value::Value;
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

impl<'a> Operation for RunCurlCommand<'a> {
    type Output = ();

    fn execute(&self, context: &OperationContext) -> crate::Result<Self::Output> {
        let env = context.environment();
        let ctx: minijinja::value::Value = env.into();
        let mut env = minijinja::Environment::new();

        self.request.placeholders.iter().for_each(|placeholder| {
            let value = placeholder
                .value
                .as_ref()
                .unwrap_or_else(|| placeholder.default.as_ref().unwrap());
            env.add_global(
                &placeholder.name,
                Value::from_safe_string(value.to_string()),
            );
        });

        let url = render(&mut env, &ctx, self.request.url.as_ref(), "url")?;
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
                    .map(|s| render(&mut env, &ctx, s, "param"))
                    .collect::<Result<Vec<_>>>()?,
            )
            .args(self.request.headers.as_ref().iter().flat_map(|(k, v)| {
                let value = render(&mut env, &ctx, v, k).unwrap();
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
