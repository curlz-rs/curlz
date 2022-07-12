use crate::data::Bookmark;
use crate::data::HttpMethod;
use crate::ops::{Operation, OperationContext};

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

impl Operation for LoadBookmark {
    type Output = Option<Bookmark>;

    fn execute(self, context: &OperationContext) -> crate::Result<Self::Output> {
        context
            .bookmark_collection()
            .load(&self.slug, &self.http_method)
    }
}
