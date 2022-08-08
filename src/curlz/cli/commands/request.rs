use crate::data::{HttpHeaders, HttpMethod, HttpRequest};
use crate::interactive;
use crate::ops::{
    LoadBookmark, MutOperation, Operation, OperationContext, RunCurlCommand, SaveBookmark,
};
use crate::utils::parse_pairs;
use crate::variables::Placeholder;

use anyhow::Context;
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
    #[clap(long, short, number_of_values = 1, value_parser)]
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
    #[clap(long = "header", short = 'H', number_of_values = 1, value_parser)]
    pub headers: Vec<String>,

    #[clap(long = "json", action)]
    pub json: bool,

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

impl MutOperation for RequestCli {
    type Output = ();

    fn execute(&self, ctx: &mut OperationContext) -> crate::Result<Self::Output> {
        let placeholders = self.parse_define_as_placeholders();
        let mut raw = self.raw.clone();

        let method = extract_method(&mut raw)
            .unwrap_or_else(|| HttpMethod::from_str(self.http_method.as_str()))?;
        let mut headers = extract_headers(&mut raw);
        if self.json {
            headers.push("Content-Type", "application/json");
            headers.push("Accept", "application/json");
        }
        let request = if let Some(bookmark_or_url) = self.bookmark_or_url.as_ref() {
            if is_url(bookmark_or_url) {
                // here we are certain we got an URL
                HttpRequest {
                    // todo: also replace placeholders in there..
                    url: bookmark_or_url.to_string(),
                    method,
                    headers,
                    placeholders,
                    // todo: implement placeholder scanning..
                    curl_params: raw,
                }
            } else {
                // here we might have a bookmark slug, but not sure yet
                let bookmark = LoadBookmark::new(bookmark_or_url, method)
                    .execute(ctx)?
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
                    headers,
                    placeholders,
                    // todo: implement placeholder scanning..
                    curl_params: raw,
                })?
        };

        RunCurlCommand::new(&request).execute(ctx)?;

        if self.save_bookmark || self.save_bookmark_as.is_some() {
            let slug = if let Some(answer) = self.save_bookmark_as.as_ref() {
                answer.clone()
            } else {
                interactive::user_question("Please enter a bookmark name", &None)?
            };

            SaveBookmark::new(slug.as_str(), &request).execute(ctx)?;

            info!("Request bookmarked as: {}", slug);
        }

        Ok(())
    }
}

/// checks if a string is a URL
fn is_url(potential_url: impl AsRef<str>) -> bool {
    potential_url
        .as_ref()
        .trim_start_matches('\'')
        .starts_with("http")
}

/// Extracts the http headers from the command line arguments
/// If a header `-H | --header` is found, it's removed from the `raw_args`
fn extract_headers(raw_args: &mut Vec<String>) -> HttpHeaders {
    let mut headers = HttpHeaders::default();
    raw_args
        .clone()
        .iter()
        .enumerate()
        .step_by(2)
        .zip(raw_args.clone().iter().enumerate().skip(1).step_by(2))
        .filter_map(|((ik, key), (iv, value))| match key.as_str() {
            "-H" | "--header" => Some(((ik, key), (iv, value))),
            _ => None,
        })
        // now it gets ugly #1
        .rev()
        .for_each(|((ki, _key), (kv, value))| {
            // ugly the #2
            raw_args.swap_remove(kv);
            raw_args.swap_remove(ki);
            let (key, value) = value.split_once(':').unwrap();
            headers.push(key.trim(), value.trim());
        });

    // ugly the #3
    headers.reverse();

    headers
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
fn extract_url(raw_args: &mut Vec<String>) -> Option<String> {
    if let Some(potential_url) = raw_args.last().cloned() {
        if potential_url.trim_start_matches('\'').starts_with("http") {
            raw_args.pop();
            Some(potential_url)
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
        assert_eq!(url, Some("http://example.com".to_string()));
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
        let mut args = vec![
            "-H",
            "foo: bar",
            "--header",
            "Accept: application/json",
            "http://example.com",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let headers = extract_headers(&mut args);
        assert_eq!(
            headers.as_ref(),
            &[
                ("foo".to_string(), "bar".to_string()),
                ("Accept".to_string(), "application/json".to_string())
            ]
        );
        assert_eq!(args.len(), 1);
        assert_eq!(args.pop(), Some("http://example.com".to_string()));
    }
}
