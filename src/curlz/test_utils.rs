use minijinja::functions::Function;
use minijinja::value::{FunctionArgs, FunctionResult, Value};
use minijinja::Environment;

/// [`RenderBuilder`] simplifies test case creation
pub struct RenderBuilder<'source> {
    env: Environment<'source>,
}

impl<'source> RenderBuilder<'source> {
    pub fn with_env_var(mut self, name: &'source str, value: impl Into<Value>) -> Self {
        self.env.add_global(name, value.into());

        self
    }
}

impl<'source> Default for RenderBuilder<'source> {
    fn default() -> Self {
        Self {
            env: Environment::empty(),
        }
    }
}

impl<'source> RenderBuilder<'source> {
    /// creates a new fresh builder
    pub fn new() -> Self {
        Self::default()
    }

    /// registers a template filter function
    pub fn with_function<F, Rv, Args>(mut self, name: &'source str, f: F) -> Self
    where
        // the crazy bounds here exist to enable borrowing in closures
        F: Function<Rv, Args> + for<'a> Function<Rv, <Args as FunctionArgs<'a>>::Output>,
        Rv: FunctionResult,
        Args: for<'a> FunctionArgs<'a>,
    {
        self.env.add_function(name, f);

        self
    }

    /// it renders a given template
    pub fn render(mut self, template: &'source str) -> String {
        let name = "something";
        self.env.add_template(name, template).unwrap();
        let template = self.env.get_template(name).unwrap();

        let ctx = Value::default();
        template.render(&ctx).unwrap()
    }
}
