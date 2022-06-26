use crate::commands::Execute;
use crate::data::HttpRequest;
use crate::CommandContext;

#[derive(Debug)]
pub struct SaveBookmarkCommand<'a> {
    pub slug: String,
    pub to_bookmark: &'a HttpRequest,
}

impl<'a> SaveBookmarkCommand<'a> {
    pub fn new(slug: impl AsRef<str>, to_bookmark: &'a HttpRequest) -> Self {
        Self {
            slug: slug.as_ref().to_owned(),
            to_bookmark,
        }
    }
}

impl<'a> Execute for SaveBookmarkCommand<'a> {
    type Output = ();

    fn execute(self, context: &CommandContext) -> crate::Result<Self::Output> {
        context.bookmark_collection().save(&(&self).into())
    }
}
