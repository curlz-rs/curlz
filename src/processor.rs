use crate::commands::BookmarkAsCommand;
use crate::request::Request;
use crate::Result;

use anyhow::anyhow;
use convert_case::{Case, Casing};
use filenamify::filenamify;
use std::path::PathBuf;
use std::{env, fs};

pub struct Processor {
    working_dir: PathBuf,
}

impl Processor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            working_dir: env::current_dir()
                .map_err(|e| anyhow!("cannot create processor: {}", e))?,
        })
    }
}

impl Processor {
    pub fn handle_bookmark_as(&self, command: &BookmarkAsCommand) -> Result<()> {
        let slug: &str = command.as_ref();
        let request: &Request = command.as_ref();

        let file_name = filenamify(format!("{:?} {}", &request.method, slug)).to_case(Case::Snake);
        let request = serde_yaml::to_string(request)?;

        // todo(refactor): extract this logic into the `BookmarkCollection` or similar
        //   - so that we only the delegation left here
        //   - tests will not have to verify the files any more, only the insta yaml snapshot tests
        let bookmarks_path = self.working_dir.join(".curlx").join("bookmarks");
        fs::create_dir_all(bookmarks_path.as_path())?;
        {
            // todo(refactor): instead of saving the request only, it would be good to save a `Bookmark`
            //    - that would contain the original slug, hence offers search capabilities
            //    - makes a clear data structural distinction between `Request` and `Bookmark<Request>`
            fs::write(
                bookmarks_path.join(format!("{}.yml", file_name.as_str())),
                request,
            )
            .map_err(|e| anyhow!("cannot write request bookmark to file: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::HttpMethod;
    use crate::{StringEntry, TemplateSlots, VarInfo};
    use regex::Regex;
    use tempfile::{tempdir, TempDir};

    impl Processor {
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
    fn should_handle_bookmark_as_command() {
        let email_var = TemplateSlots {
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
        };
        let cmd = BookmarkAsCommand::new(
            "/protonmail/gpg/{{email}}",
            Request {
                url: "https://api.protonmail.ch/pks/lookup?op=get&search={{email}}".to_owned(),
                method: HttpMethod::Get,
                curl_params: vec![],
                placeholders: vec![email_var],
            },
        );

        let (p, tmp) = Processor::temporary();
        p.handle_bookmark_as(&cmd).unwrap();

        let saved_bookmark = String::from_utf8(
            fs::read(
                tmp.path()
                    .join(".curlx")
                    .join("bookmarks")
                    .join("get_protonmail_gpg_{{email}}.yml"),
            )
            .unwrap(),
        )
        .unwrap();

        insta::assert_snapshot!(saved_bookmark);
    }
}
