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
        let bookmarks_path = self.working_dir.join(".curlx").join("bookmarks");

        fs::create_dir_all(bookmarks_path.as_path())?;
        {
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

        let p = Processor::new().unwrap();
        p.handle_bookmark_as(&cmd).unwrap();

        assert!(fs::metadata(
            std::env::current_dir()
                .unwrap()
                .join(".curlx")
                .join("bookmarks")
                .join("get_protonmail_gpg_{{email}}.yml")
        )
        .unwrap()
        .is_file());
    }
}
