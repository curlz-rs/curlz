use crate::data::HttpRequest;
use crate::ops::Operation;

use super::OperationContext;

#[derive(Debug)]
pub struct SaveBookmark<'a> {
    pub slug: String,
    pub bookmark: &'a HttpRequest,
}

impl<'a> SaveBookmark<'a> {
    pub fn new(slug: impl AsRef<str>, bookmark: &'a HttpRequest) -> Self {
        Self {
            slug: slug.as_ref().to_owned(),
            bookmark,
        }
    }
}

impl<'a> Operation for SaveBookmark<'a> {
    type Output = ();

    fn execute(&self, context: &OperationContext) -> crate::Result<Self::Output> {
        context.bookmark_collection().save(&(self).into())
    }
}
