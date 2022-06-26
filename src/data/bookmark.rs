use serde::{Deserialize, Serialize};

use crate::{HttpRequest, SaveBookmarkCommand};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    slug: String,
    request: HttpRequest,
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

impl<'a> From<&SaveBookmarkCommand<'a>> for Bookmark {
    fn from(cmd: &SaveBookmarkCommand) -> Self {
        Self {
            slug: cmd.slug.clone(),
            request: cmd.to_bookmark.clone(),
        }
    }
}
