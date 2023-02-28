use crate::domain::bookmark::{Bookmark, BookmarkCollection};
use crate::domain::http::HttpMethod;

#[derive(Debug)]
pub struct LoadBookmark {
    pub slug: String,
    pub http_method: HttpMethod,
}

impl LoadBookmark {
    pub fn new(slug: impl AsRef<str>, http_method: HttpMethod) -> Self {
        Self {
            slug: slug.as_ref().to_owned(),
            http_method,
        }
    }
}

pub fn load_bookmark(bm: LoadBookmark, collection: &impl BookmarkCollection) -> Option<Bookmark> {
    collection.load(bm.slug, &bm.http_method).unwrap()
}
