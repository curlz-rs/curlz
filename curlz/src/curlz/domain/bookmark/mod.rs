mod collection;
mod collection_impl;
mod load_bookmark;
mod save_bookmark;

pub use self::collection::BookmarkCollection;
pub use self::collection_impl::BookmarkFolderCollection;
pub use self::load_bookmark::*;
pub use self::save_bookmark::*;

use crate::domain::http::HttpRequest;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bookmark {
    pub slug: String,
    pub request: HttpRequest,
}

impl AsRef<HttpRequest> for Bookmark {
    fn as_ref(&self) -> &HttpRequest {
        &self.request
    }
}

impl AsRef<str> for Bookmark {
    fn as_ref(&self) -> &str {
        self.slug.as_ref()
    }
}

impl Bookmark {
    pub fn slug(&self) -> &str {
        self.as_ref()
    }

    pub fn request(&self) -> &HttpRequest {
        self.as_ref()
    }
}

impl<'a> From<&SaveBookmark<'a>> for Bookmark {
    fn from(cmd: &SaveBookmark<'a>) -> Self {
        Self {
            slug: cmd.slug.clone(),
            request: cmd.bookmark.clone(),
        }
    }
}
