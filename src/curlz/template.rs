use crate::interactive::prompt_password;

use minijinja::value::Value;
use minijinja::Environment;

pub struct Renderer<'source> {
    env: Environment<'source>,
    ctx: Value,
}

impl<'source> From<&crate::workspace::Environment> for Renderer<'source> {
    fn from(env: &crate::workspace::Environment) -> Self {
        Self::new(env)
    }
}

impl<'source> Renderer<'source> {
    pub fn new(env: &crate::workspace::Environment) -> Self {
        let ctx: Value = env.into();
        let mut env = Environment::new();
        env.add_function("prompt_password", prompt_password);

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
