use crate::domain::environment::create_environment;
use crate::domain::http_lang::parse_request_file;
use crate::domain::request::Verbosity::Verbose;
use crate::domain::request::{issue_request_with_curl, IssueRequest};
use crate::template::variables::Placeholder;
use crate::utils::parse_pairs;
use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct HttpFileCli {
    /// Provide an `.env` or a yaml containing template variables
    #[clap(long = "env-file", value_parser, default_value = ".env")]
    pub env_file: PathBuf,

    /// Define a adhoc template variable like `--define foo=value --define bar=42`, see also `--env-file` for more convenience
    #[clap(long, number_of_values = 1, value_parser)]
    pub define: Vec<String>,

    /// Provide an http request file
    #[clap(value_parser)]
    pub http_file: PathBuf,
}

impl HttpFileCli {
    pub fn execute(&self) -> crate::Result<()> {
        let placeholders: Vec<Placeholder> = self
            .define
            .iter()
            .map(|kv| parse_pairs(kv, '='))
            .filter(Option::is_some)
            .flatten()
            .map(|(key, value)| Placeholder::new(key, value))
            .collect();
        let env = create_environment(&self.env_file, &placeholders)?;
        let contents = std::fs::read_to_string(&self.http_file)?;
        let bookmarks = parse_request_file(contents)?;

        for b in bookmarks {
            issue_request_with_curl(IssueRequest::new(&b.request, Verbose), &env)?;
        }

        Ok(())
    }
}
