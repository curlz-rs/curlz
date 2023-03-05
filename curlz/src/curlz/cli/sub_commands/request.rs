use crate::cli::interactive;
use crate::cli::HeaderArgs;
use crate::domain::bookmark::{
    load_bookmark, save_bookmark, BookmarkCollection, BookmarkFolderCollection, LoadBookmark,
    SaveBookmark,
};
use crate::domain::http::{
    HttpBody, HttpHeaders, HttpMethod, HttpRequest, HttpUri, HttpVersion::Http11,
};
use crate::domain::request::Verbosity::{Silent, Verbose};
use crate::domain::request::{issue_request_with_curl, IssueRequest};
use crate::template::variables::Placeholder;
use crate::utils::parse_pairs;

use crate::domain::environment::create_environment;
use anyhow::{bail, Context};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::info;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct RequestCli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[clap(long = "bookmark-as", value_parser)]
    pub save_bookmark_as: Option<String>,

    #[clap(long = "bookmark", action)]
    pub save_bookmark: bool,

    /// Provide an `.env` or a yaml containing template variables
    #[clap(long = "env-file", value_parser, default_value = ".env")]
    pub env_file: PathBuf,

    /// Define a adhoc template variable like `--define foo=value --define bar=42`, see also `--env-file` for more convenience
    #[clap(long, number_of_values = 1, value_parser)]
    pub define: Vec<String>,

    #[clap(short = 'X', long = "request", value_parser, default_value = "GET")]
    pub http_method: String,

    /// set one ore more http headers in the form of `"Header-Name: Value"`
    ///
    /// ## Examples
    /// ```sh
    /// curlz -H "X-First-Name: Joe" https://example.com
    /// curlz -H "User-Agent: yes-please/2000" https://example.com
    /// curlz -H "Host:" https://example.com
    /// ```
    #[clap(long = "header", short = 'H', value_parser)]
    pub headers: Vec<String>,

    /// set a http body
    #[clap(short = 'd', long = "data", value_parser)]
    pub http_payload: Option<String>,

    /// this is a lazy shortcut for setting 2 headers and a http body
    /// ```sh
    /// curlz -H "Content-Type: application/json" -H "Accept: application/json" --data <json-data>
    /// ```
    #[clap(long = "json", value_parser)]
    pub json: Option<String>,

    /// <user:password>
    /// Specify the user name and password to use for server authentication.
    /// Note: in cases where only the user is provided,
    ///       curlz will prompt for the password interactively
    /// Equivalent to:
    /// ```sh
    /// curlz -H 'Authorization: Basic {{ basic("user", "password") }}'
    /// ```
    #[clap(short = 'u', long = "user", value_parser)]
    pub user: Option<String>,

    #[clap(value_parser)]
    pub bookmark_or_url: Option<String>,

    #[clap(value_parser, last = true, multiple = true)]
    pub raw: Vec<String>,
}

impl RequestCli {
    pub fn parse_define_as_placeholders(&self) -> Vec<Placeholder> {
        self.define
            .iter()
            .map(|kv| parse_define(kv.as_str()))
            .filter(Option::is_some)
            .flatten()
            .map(|(key, value)| Placeholder::new(key, value))
            .collect()
    }
}

/// parses `key=value` strings into tuples of (key, value)
#[inline]
fn parse_define(define: &str) -> Option<(&str, &str)> {
    parse_pairs(define, '=')
}

impl RequestCli {
    pub fn execute(&self) -> crate::Result<()> {
        let placeholders = self.parse_define_as_placeholders();
        let env = create_environment(&self.env_file, &placeholders)?;
        let mut raw = self.raw.clone();

        let method = extract_method(&mut raw)
            .unwrap_or_else(|| HttpMethod::from_str(self.http_method.as_str()))?;

        // headers
        let mut headers: HttpHeaders = self.headers.as_slice().into();
        let (mut raw, headers_args) = extract_headers(&raw);
        let headers_raw: HttpHeaders = headers_args.into();
        headers.merge(&headers_raw);
        if self.json.is_some() {
            headers.push("Content-Type", "application/json");
            headers.push("Accept", "application/json");
        }
        if self.user.is_some() {
            let user_pw: Vec<&str> = self.user.as_ref().unwrap().split_terminator(':').collect();
            let header_value = match user_pw.len() {
                1 => {
                    format!(
                        r#"Basic {{{{ basic("{}", prompt_password()) }}}}"#,
                        user_pw.first().unwrap()
                    )
                }
                2 => {
                    format!(
                        r#"Basic {{{{ basic("{}", "{}") }}}}"#,
                        user_pw.first().unwrap(),
                        user_pw.get(1).unwrap()
                    )
                }
                _ => bail!("-u | -user argument was invalid"),
            };
            headers.push("Authorization", header_value);
        }

        let body = self
            .http_payload
            .as_ref()
            .map(|b| HttpBody::InlineText(b.to_string()))
            .or_else(|| self.json.clone().map(HttpBody::InlineText))
            .unwrap_or_default();

        let request = if let Some(bookmark_or_url) = self.bookmark_or_url.as_ref() {
            if is_url(bookmark_or_url) {
                // here we are certain we got an URL
                HttpRequest {
                    // todo: also replace placeholders in there..
                    url: bookmark_or_url.to_string().try_into()?,
                    method,
                    version: Http11,
                    headers,
                    body,
                    placeholders,
                    // todo: implement placeholder scanning..
                    curl_params: raw,
                }
            } else {
                let bookmark_collection = bookmark_collection()?;
                // here we might have a bookmark slug, but not sure yet
                let bookmark = load_bookmark(
                    LoadBookmark::new(bookmark_or_url, method),
                    &bookmark_collection,
                )
                .context("No Bookmark with the given name found")?;

                bookmark.request().update(|request| {
                    request.headers.merge(&headers);
                    request.curl_params.extend_from_slice(&raw);
                })
            }
        } else {
            // try to extract an URL from the raw args provided
            extract_url(&mut raw)
                .context("Raw arguments did not contain any URL")
                .map(|url| HttpRequest {
                    url,
                    method,
                    version: Http11,
                    headers,
                    body,
                    placeholders,
                    // todo: implement placeholder scanning..
                    curl_params: raw,
                })?
        };

        issue_request_with_curl(
            IssueRequest::new(
                &request,
                if self.verbose.is_silent() {
                    Silent
                } else {
                    Verbose
                },
            ),
            &env,
        )?;

        if self.save_bookmark || self.save_bookmark_as.is_some() {
            let slug = if let Some(answer) = self.save_bookmark_as.as_ref() {
                answer.clone()
            } else {
                interactive::user_question("Please enter a bookmark name", &None)?
            };

            let mut bookmark_collection = bookmark_collection()?;
            save_bookmark(
                SaveBookmark::new(slug.as_str(), &request),
                &mut bookmark_collection,
            )?;

            info!("Request bookmarked as: {}", slug);
        }

        Ok(())
    }
}

fn bookmark_collection() -> crate::Result<impl BookmarkCollection> {
    BookmarkFolderCollection::new()
}

/// checks if a string is a URL
fn is_url(potential_url: impl AsRef<str>) -> bool {
    let trimmed_url = potential_url.as_ref().trim_start_matches('\'');

    trimmed_url.starts_with("http") || trimmed_url.starts_with("{{")
}

/// Extracts the http headers from command line arguments
/// If a header `-H | --header` is found, it's removed from the `raw_args`
fn extract_headers(raw_args: &Vec<String>) -> (Vec<String>, HeaderArgs) {
    let headers = HeaderArgs::from(raw_args);

    let mut non_header_args = vec![];
    let mut i = 0_usize;
    while i < raw_args.len() {
        match raw_args.get(i).unwrap().as_str() {
            "-H" | "--header" => {
                i += 2;
            }
            v => {
                non_header_args.push(v.to_string());
                i += 1;
            }
        }
    }

    (non_header_args, headers)
}

/// Extracts the http method from the command line arguments
/// If a method `-X | --request` is found, it's removed from the `raw_args`
///
/// ## Fallible
/// If the method is provided but invalid, an error is returned
///
/// ## None
/// simple: in case no http method is provided, None is returned
fn extract_method(raw_args: &mut Vec<String>) -> Option<crate::Result<HttpMethod>> {
    let mut method = None;

    let copy = raw_args.clone();
    copy.iter()
        .enumerate()
        .step_by(2)
        .zip(copy.iter().enumerate().skip(1).step_by(2))
        .for_each(|((ik, key), (iv, value))| {
            if key.as_str().eq("-X") || key.as_str().eq("--request") {
                raw_args.remove(ik);
                raw_args.remove(iv);
                method = Some(HttpMethod::from_str(value.as_str()));
            }
        });

    method
}

/// extracts a `http://` or `https://` URL from the command line arguments `raw_args`
/// if a URL is found it's removed from the `raw_args` vector and returned
/// If no URL is found, returns `None`
fn extract_url(raw_args: &mut Vec<String>) -> Option<HttpUri> {
    if let Some(potential_url) = raw_args.last().cloned() {
        if potential_url.trim_start_matches('\'').starts_with("http") {
            raw_args.pop();
            // todo: no unwrap here:
            Some(potential_url.try_into().unwrap())
        } else if potential_url.starts_with("{{") {
            todo!("placeholder evaluation at the beginning of URLs")
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_defines_by_equal() {
        assert_eq!(parse_define("foo=bar"), Some(("foo", "bar")));
        assert_eq!(parse_define("foo"), None);
        assert_eq!(parse_define("baz="), Some(("baz", "")));
    }

    #[test]
    fn should_extract_a_url_as_last_argument() {
        let mut args = vec!["--request", "GET", "http://example.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let url = extract_url(&mut args);
        assert_eq!(
            url,
            Some("http://example.com".to_string().try_into().unwrap())
        );
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn should_extract_method() {
        let mut args = vec!["--request", "GET", "http://example.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let method = extract_method(&mut args).unwrap().unwrap();
        assert_eq!(method, HttpMethod::Get);
        assert_eq!(args.len(), 1);
    }

    #[test]
    fn should_extract_headers() {
        let args = vec![
            "-vvv",
            "-H",
            "foo: bar",
            "--header",
            "Accept: application/json",
            "--header",
            r#"Authorization: Baerer {{ jwt({"foo": "bar"}) }}"#,
            "http://example.com",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        let (args, headers) = extract_headers(&args);
        assert_eq!(
            headers.as_ref(),
            &[
                "foo: bar".to_string(),
                "Accept: application/json".to_string(),
                r#"Authorization: Baerer {{ jwt({"foo": "bar"}) }}"#.to_string(),
            ]
        );
        // TODO: it's unclear why this here fails:
        assert_eq!(args.len(), 2);
        assert_eq!(args.first(), Some(&"-vvv".to_string()));
        assert_eq!(args.last(), Some(&"http://example.com".to_string()));
    }
}
