use crate::domain::bookmark::collection::BookmarkCollection;
use crate::domain::bookmark::Bookmark;
use crate::domain::http::HttpRequest;

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

pub fn save_bookmark(
    bm: SaveBookmark,
    collection: &mut impl BookmarkCollection,
) -> crate::Result<()> {
    collection.save(&Bookmark::from(&bm))
}
