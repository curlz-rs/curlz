use crate::cli::Cli;
use crate::commands::{Execute, RunCurlCommand, SaveBookmarkCommand};
use crate::data::{HttpHeaders, HttpMethod, HttpRequest};
use crate::processor::CommandContext;
use crate::variables::TemplateSlots;
use crate::workspace::Environment;

use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

mod cli;
mod commands;
mod data;
mod interactive;
mod processor;
mod utils;
mod variables;
mod workspace;

pub type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let Cli {
        define,
        env_file,
        http_method,
        bookmark,
        json,
        mut raw,
        save_bookmark_as,
        save_bookmark,
        ..
    } = Cli::parse();

    let url = extract_url(bookmark, &mut raw).context("No Bookmark nor URL was provided")?;
    let method =
        extract_method(&mut raw).unwrap_or_else(|| HttpMethod::from_str(http_method.as_str()))?;
    let mut headers = extract_headers(&mut raw);

    let env = create_environment(env_file, define)?;
    let ctx = CommandContext::new(env)?;

    if json {
        headers.push("Content-Type", "application/json");
        headers.push("Accept", "application/json");
    }

    let request = HttpRequest {
        url,
        method,
        headers,
        curl_params: raw,
        // todo: implement placeholder scanning..
        placeholders: vec![],
    };
    RunCurlCommand::new(&request).execute(&ctx)?;

    if save_bookmark || save_bookmark_as.is_some() {
        let slug = if let Some(answer) = save_bookmark_as {
            answer
        } else {
            println!("Saving this request as a bookmark:");
            interactive::user_question("  Please enter a name", &None)?
        };

        SaveBookmarkCommand::new(slug.as_str(), &request).execute(&ctx)?;

        println!("This request is bookmarked as: {}", slug.as_str());
    }

    Ok(())
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
fn extract_method(raw_args: &mut Vec<String>) -> Option<Result<HttpMethod>> {
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
fn extract_url(bookmark: Option<String>, raw_args: &mut Vec<String>) -> Option<String> {
    bookmark.or_else(|| {
        if let Some(potential_url) = raw_args.last().cloned() {
            if potential_url.starts_with("http") {
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
    })
}

/// creates an [`Environment`] from a `.env` file or a `.yaml` file
/// If the file does not exist, an empty [`Environment`] is returned.
///
/// ## Fallible
/// If `env_file` is not a `.env` or `.yaml` file, an error is returned.
/// If `env_file` is a directory, an error is returned.
fn create_environment(env_file: PathBuf, define: Vec<String>) -> Result<Environment> {
    Environment::try_from(env_file.as_path()).map(|mut env| {
        parse_define(&define).for_each(|(k, v)| env.insert(k, v));
        env
    })
}

/// parses `key=value` strings into tuples of (key, value)
fn parse_define(define: &[impl AsRef<str>]) -> impl Iterator<Item = (&str, &str)> {
    define
        .iter()
        .map(|var| {
            var.as_ref()
                .split_once('=')
                .map(|(key, value)| (key.trim(), value.trim()))
        })
        .filter(Option::is_some)
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_defines_by_equal_happy_path() {
        let defines = vec!["foo=bar", "baz=qux"];
        let mut iter = parse_define(&defines);
        assert_eq!(iter.next(), Some(("foo", "bar")));
        assert_eq!(iter.next(), Some(("baz", "qux")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_split_defines_by_equal_unhappy_path() {
        let defines = vec!["baz=123324+adf+=vasdf"];
        let mut iter = parse_define(&defines);
        assert_eq!(iter.next(), Some(("baz", "123324+adf+=vasdf")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_not_split_defines_if_no_equal_is_contained() {
        let defines = vec!["foo", "baz="];
        let mut iter = parse_define(&defines);
        assert_eq!(iter.next(), Some(("baz", "")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_extract_a_url_as_last_argument() {
        let mut args = vec!["--request", "GET", "http://example.com"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let url = extract_url(None, &mut args);
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
