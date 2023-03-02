mod functions;
pub mod variables;

use functions::auth::jwt;
use functions::process_env::process_env;
use functions::prompt::{prompt_for, prompt_password};

use crate::domain::environment::Environment;
use crate::template::functions::process_env::ProcessEnv;

use minijinja::value::Value;
use minijinja::Environment as MEnvironment;

pub struct Renderer<'source> {
    env: MEnvironment<'source>,
    ctx: Value,
}

impl<'source> From<&Environment> for Renderer<'source> {
    fn from(env: &Environment) -> Self {
        Self::new(env)
    }
}

impl<'source> Renderer<'source> {
    pub fn new(env: &Environment) -> Self {
        let ctx: Value = env.into();
        let mut env = MEnvironment::new();
        env.add_function("processEnv", process_env);
        env.add_function("process_env", process_env);

        env.add_function("prompt_password", prompt_password);
        env.add_function("prompt_for", prompt_for);
        env.add_function("jwt", jwt);

        // this provides lazy env var lookup
        env.add_global("env", Value::from_struct_object(ProcessEnv));

        Self { env, ctx }
    }

    pub fn inject_variable(&mut self, p0: &'source str, p1: String) {
        self.env.add_global(p0, Value::from_safe_string(p1));
    }

    pub fn render(&mut self, str: &'source str, name: &'source str) -> crate::Result<String> {
        self.env.add_template(name, str)?;
        let template = self.env.get_template(name)?;

        template.render(&self.ctx).map_err(|e| e.into())
    }
}
