pub use load_bookmark::*;
pub use run_curl::*;
pub use save_bookmark::*;

use crate::workspace::{BookmarkCollection, Environment};
use crate::Result;

mod load_bookmark;
mod run_curl;
mod save_bookmark;

/// represents an executable operation
pub trait Operation {
    type Output;

    fn execute(self, context: &OperationContext) -> Result<Self::Output>;
}

pub trait MutOperation {
    type Output;

    fn execute(self, context: &mut OperationContext) -> Result<Self::Output>;
}

/// processes all commands and keeps the application state
pub struct OperationContext {
    bookmark_collection: BookmarkCollection,
    environment: Environment,
}

impl OperationContext {
    /// ## fallible
    /// in cases where the workspace folder is not accessible
    pub fn new(env: Environment) -> Result<Self> {
        BookmarkCollection::new().map(|bookmark_collection| Self {
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
}
