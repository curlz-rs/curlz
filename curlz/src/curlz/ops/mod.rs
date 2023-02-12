pub use run_curl::*;

use crate::template::Renderer;
use crate::variables::Placeholder;
use crate::workspace::{BookmarkCollection, Environment};
use crate::Result;

mod run_curl;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Verbosity {
    Silent,
    Verbose,
}

/// represents an executable operation
pub trait Operation {
    type Output;

    fn execute(&self, context: &OperationContext) -> Result<Self::Output>;
}

pub trait MutOperation {
    type Output;

    fn execute(&self, context: &mut OperationContext) -> Result<Self::Output>;
}

/// processes all commands and keeps the application state
pub struct OperationContext {
    pub verbosity: Verbosity,
    bookmark_collection: BookmarkCollection,
    environment: Environment,
}

impl OperationContext {
    /// ## fallible
    /// in cases where the workspace folder is not accessible
    pub fn new(env: Environment, verbosity: Verbosity) -> Result<Self> {
        BookmarkCollection::new().map(|bookmark_collection| Self {
            verbosity,
            bookmark_collection,
            environment: env,
        })
    }

    pub fn bookmark_collection(&self) -> &BookmarkCollection {
        &self.bookmark_collection
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn environment_mut(&mut self) -> &mut Environment {
        &mut self.environment
    }

    /// creates a new rendere based on the inner ['Environment`]
    pub fn renderer(&self) -> Renderer {
        (&self.environment).into()
    }

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
