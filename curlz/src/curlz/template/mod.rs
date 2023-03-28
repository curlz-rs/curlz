mod functions;
pub mod variables;

use crate::domain::environment::Environment;

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

        functions::register_functions(&mut env);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_from_environment() {
        let mut env = Environment::default();
        env.insert("foo", "bar");
        let mut r: Renderer = (&env).into();
        r.inject_variable("bak", "foo".to_string());

        assert_eq!(r.render("{{ foo }}", "something").unwrap(), "bar");
        assert_eq!(r.render("{{ bak }}", "something2").unwrap(), "foo");
    }
}
