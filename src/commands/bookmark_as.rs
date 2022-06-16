use crate::request::Request;

#[derive(Debug)]
pub struct BookmarkAsCommand {
    slug: String,
    to_bookmark: Request,
}

impl BookmarkAsCommand {
    pub fn new(slug: impl AsRef<str>, to_bookmark: Request) -> Self {
        Self {
            slug: slug.as_ref().to_owned(),
            to_bookmark,
        }
    }
}

impl AsRef<Request> for BookmarkAsCommand {
    fn as_ref(&self) -> &Request {
        &self.to_bookmark
    }
}

impl AsRef<str> for BookmarkAsCommand {
    fn as_ref(&self) -> &str {
        self.slug.as_ref()
    }
}
