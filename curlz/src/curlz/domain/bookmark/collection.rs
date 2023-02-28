use crate::domain::bookmark::Bookmark;
use crate::domain::http::HttpMethod;
use crate::Result;

pub trait BookmarkCollection {
    fn save(&self, bookmark: &Bookmark) -> Result<()>;
    fn load(&self, name: impl AsRef<str>, method: &HttpMethod) -> Result<Option<Bookmark>>;
}
