use anyhow::anyhow;
use convert_case::{Case, Casing};
use filenamify::filenamify;
use std::path::PathBuf;
use std::{env, fs};

use crate::domain::bookmark::collection::BookmarkCollection;
use crate::domain::bookmark::Bookmark;
use crate::domain::http::HttpMethod;
use crate::Result;

const WORKSPACE_FOLDER: &str = ".curlz";
const BOOKMARK_FOLDER: &str = "bookmarks";

pub struct BookmarkFolderCollection {
    working_dir: PathBuf,
}

impl BookmarkFolderCollection {
    pub fn new() -> Result<Self> {
        Ok(Self {
            working_dir: env::current_dir()
                .map_err(|e| anyhow!("cannot create processor: {}", e))?,
        })
    }
}

impl BookmarkCollection for BookmarkFolderCollection {
    fn save(&self, bookmark: &Bookmark) -> Result<()> {
        let slug = bookmark.slug();
        let request = bookmark.request();

        let file_name = filenamify(format!("{:?} {}", &request.method, slug)).to_case(Case::Snake);
        let bookmark = serde_yaml::to_string(&bookmark)?;

        let bookmarks_path = self
            .working_dir
            .join(WORKSPACE_FOLDER)
            .join(BOOKMARK_FOLDER);
        fs::create_dir_all(bookmarks_path.as_path())?;
        {
            fs::write(
                bookmarks_path.join(format!("{}.yml", file_name.as_str())),
                bookmark,
            )
            .map_err(|e| anyhow!("cannot write request bookmark to file: {}", e))
        }
    }

    fn load(&self, name: impl AsRef<str>, method: &HttpMethod) -> Result<Option<Bookmark>> {
        let slug = name.as_ref();
        let bookmarks_path = self
            .working_dir
            .join(WORKSPACE_FOLDER)
            .join(BOOKMARK_FOLDER);
        let file_name = filenamify(format!("{:?} {}", method, slug)).to_case(Case::Snake);
        let file_path = bookmarks_path.join(format!("{}.yml", file_name.as_str()));
        if !file_path.exists() {
            return Ok(None);
        }
        let bookmark = fs::read_to_string(file_path)?;
        Ok(Some(serde_yaml::from_str(&bookmark)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::bookmark::SaveBookmark;
    use crate::domain::http::HttpVersion::Http11;
    use crate::domain::http::{HttpBody, HttpMethod};
    use crate::domain::http::{HttpHeaders, HttpRequest};
    use crate::template::variables::Placeholder;
    use tempfile::{tempdir, TempDir};

    impl BookmarkFolderCollection {
        pub fn temporary() -> (Self, TempDir) {
            let tempdir = tempdir().unwrap();
            (
                Self {
                    working_dir: tempdir.path().to_path_buf(),
                },
                tempdir,
            )
        }
    }

    #[test]
    fn should_handle_save_bookmark_command() {
        let request = HttpRequest {
            url: "{{protonmail_api_baseurl}}/pks/lookup?op=get&search={{email}}".into(),
            method: HttpMethod::Get,
            version: Http11,
            headers: HttpHeaders::default(),
            body: HttpBody::default(),
            curl_params: vec![],
            placeholders: vec![email_placeholder(), protonmail_api_baseurl_placeholder()],
        };
        let cmd = SaveBookmark::new("/protonmail/gpg/:email", &request);

        let (p, tmp) = BookmarkFolderCollection::temporary();
        p.save(&(&cmd).into()).unwrap();

        let saved_bookmark = String::from_utf8(
            fs::read(
                tmp.path()
                    .join(WORKSPACE_FOLDER)
                    .join(BOOKMARK_FOLDER)
                    .join("get_protonmail_gpg_email.yml"),
            )
            .unwrap(),
        )
        .unwrap();

        insta::assert_snapshot!(saved_bookmark);
    }

    fn email_placeholder() -> Placeholder {
        Placeholder {
            name: "email".to_string(),
            value: None,
            default: None,
            prompt: "enter an email address".to_string().into(),
        }
    }

    fn protonmail_api_baseurl_placeholder() -> Placeholder {
        Placeholder {
            name: "protonmail_api_baseurl".to_string(),
            value: None,
            default: "https://api.protonmail.ch".to_string().into(),
            prompt: "enter the protonmail api baseurl".to_string().into(),
        }
    }
}
