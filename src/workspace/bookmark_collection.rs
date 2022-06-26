use anyhow::anyhow;
use convert_case::{Case, Casing};
use filenamify::filenamify;
use std::path::PathBuf;
use std::{env, fs};

use crate::data::Bookmark;
use crate::Result;

const WORKSPACE_FOLDER: &str = ".curlx";
const BOOKMARK_FOLDER: &str = "bookmarks";

pub struct BookmarkCollection {
    working_dir: PathBuf,
}

impl BookmarkCollection {
    pub fn new() -> Result<Self> {
        Ok(Self {
            working_dir: env::current_dir()
                .map_err(|e| anyhow!("cannot create processor: {}", e))?,
        })
    }
}

impl BookmarkCollection {
    pub fn save(&self, bookmark: &Bookmark) -> Result<()> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::HttpMethod;
    use crate::variables::{StringEntry, VarInfo};
    use crate::{HttpHeaders, HttpRequest, SaveBookmarkCommand, TemplateSlots};
    use regex::Regex;
    use tempfile::{tempdir, TempDir};

    impl BookmarkCollection {
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
            url: "{{protonmail_api_baseurl}}/pks/lookup?op=get&search={{email}}".to_owned(),
            method: HttpMethod::Get,
            headers: HttpHeaders::default(),
            curl_params: vec![],
            placeholders: vec![email_placeholder(), protonmail_api_baseurl_placeholder()],
        };
        let cmd = SaveBookmarkCommand::new("/protonmail/gpg/:email", &request);

        let (p, tmp) = BookmarkCollection::temporary();
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

    fn email_placeholder() -> TemplateSlots {
        TemplateSlots {
            var_name: "email".to_string(),
            var_info: VarInfo::String {
                entry: Box::new(StringEntry {
                    default: None,
                    choices: None,
                    regex: Some(
                        Regex::new(r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$")
                            .unwrap(),
                    ),
                }),
            },
            prompt: "enter an email address".to_string(),
        }
    }

    fn protonmail_api_baseurl_placeholder() -> TemplateSlots {
        TemplateSlots {
            var_name: "protonmail_api_baseurl".to_string(),
            var_info: VarInfo::String {
                entry: Box::new(StringEntry {
                    default: Some("https://api.protonmail.ch".to_string()),
                    choices: None,
                    regex: None,
                }),
            },
            prompt: "enter the protonmail api baseurl".to_string(),
        }
    }
}
