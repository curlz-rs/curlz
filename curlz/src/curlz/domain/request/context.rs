use crate::domain::environment::Environment;
use crate::template::variables::Placeholder;
use crate::template::Renderer;

/// processes all commands and keeps the application state
pub struct RequestContext<'a> {
    environment: &'a Environment,
}

impl<'a> RequestContext<'a> {
    pub fn new(environment: &'a Environment) -> Self {
        Self { environment }
    }

    pub fn environment(&self) -> &Environment {
        self.environment
    }

    /// creates a new renderer based on the inner ['Environment`]
    pub fn renderer(&self) -> Renderer {
        self.environment.into()
    }

    /// creates a new renderer based on the inner [`Environment`]
    /// and the provided `placeholders`
    pub fn renderer_with_placeholders<'source>(
        &'source self,
        placeholders: &'source [Placeholder],
    ) -> Renderer<'source> {
        let mut r = self.renderer();

        placeholders.iter().for_each(|placeholder| {
            let value = placeholder
                .value
                .as_ref()
                .unwrap_or_else(|| placeholder.default.as_ref().unwrap());

            r.inject_variable(&placeholder.name, value.to_string());
        });

        r
    }
}
