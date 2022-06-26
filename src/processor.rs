use crate::workspace::BookmarkCollection;
use crate::{Environment, Result};

/// processes all commands and keeps the application state
pub struct CommandContext {
    bookmark_collection: BookmarkCollection,
    enviroment: Environment,
}

impl CommandContext {
    /// ## fallible
    /// in cases where the workspace folder is not accessible
    pub fn new(env: Environment) -> Result<Self> {
        BookmarkCollection::new().map(|bookmark_collection| Self {
            bookmark_collection,
            enviroment: env,
        })
    }

    pub fn bookmark_collection(&self) -> &BookmarkCollection {
        &self.bookmark_collection
    }

    pub(crate) fn environment(&self) -> &Environment {
        &self.enviroment
    }
}
