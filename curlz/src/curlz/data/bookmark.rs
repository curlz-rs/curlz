use serde::{Deserialize, Serialize};

use crate::data::HttpRequest;
use crate::ops::SaveBookmark;

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
    fn from(cmd: &SaveBookmark) -> Self {
        Self {
            slug: cmd.slug.clone(),
            request: cmd.bookmark.clone(),
        }
    }
}
