//! a module for experimenting with the http language that the rest client uses
use crate::domain::bookmark::Bookmark;
use crate::domain::http::{HttpBody, HttpHeaders, HttpMethod, HttpRequest, HttpUri, HttpVersion};

use anyhow::anyhow;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "curlz/domain/http_lang/http-lang-grammar.pest"] // relative to project `src`
struct HttpParser;

#[inline]
fn trimmed_string<T: pest::RuleType>(rule: Pair<'_, T>) -> String {
    rule.as_str()
        .trim()
        .chars()
        .filter(|x| !['\n', '\r'].contains(x))
        .collect()
}

pub fn parse_request_file(req_file: impl AsRef<str>) -> Result<Vec<Bookmark>, anyhow::Error> {
    let mut requests = vec![];

    let req_file = req_file.as_ref();
    let file = HttpParser::parse(Rule::file, req_file)?.next().unwrap();

    let mut delimiter: String = "".to_owned();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::request => {
                requests.push(Bookmark {
                    slug: delimiter.to_owned(),
                    request: HttpRequest::try_from(line)?,
                });
            }
            Rule::delimiter => delimiter = trimmed_string(line),
            Rule::EOI => {}
            x => {
                todo!("x = {:?}\n", x);
            }
        }
    }

    Ok(requests)
}

/// todo: write tests
impl TryFrom<Pair<'_, Rule>> for HttpHeaders {
    type Error = anyhow::Error;

    fn try_from(headers: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match headers.as_rule() {
            Rule::headers => {
                let mut h: HttpHeaders = Default::default();
                for header in headers.into_inner() {
                    let mut inner_rules = header.into_inner();

                    let name = trimmed_string(inner_rules.next().unwrap());
                    let value = trimmed_string(inner_rules.next().unwrap());

                    h.push(name, value);
                }
                Ok(h)
            }
            _ => Err(anyhow!("The parsing result are not a valid `headers`")),
        }
    }
}

/// todo: write tests
impl TryFrom<Pair<'_, Rule>> for HttpRequest {
    type Error = anyhow::Error;
    fn try_from(request: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match request.as_rule() {
            Rule::request => {
                let mut inner_rules = request.into_inner();
                let method: HttpMethod = inner_rules.next().unwrap().try_into()?;
                let url: HttpUri = inner_rules.next().unwrap().try_into()?;
                let version: HttpVersion = inner_rules.next().unwrap().try_into()?;
                let headers = inner_rules
                    .next()
                    .map(HttpHeaders::try_from)
                    .unwrap_or_else(|| Ok(HttpHeaders::default()))?;
                let body = inner_rules
                    .next()
                    .map(HttpBody::try_from)
                    // todo: maybe an error on parsing remains an error
                    .map(|b| b.unwrap_or_default())
                    .unwrap_or_default();

                Ok(Self {
                    url,
                    method,
                    version,
                    headers,
                    body,
                    curl_params: Default::default(),
                    placeholders: Default::default(),
                })
            }
            _ => Err(anyhow!("The parsing result is not a valid `request`")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for HttpUri {
    type Error = anyhow::Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::uri => value.as_str().to_string().try_into(),
            _ => Err(anyhow!("The parsing result is not a valid `uri`")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for HttpMethod {
    type Error = anyhow::Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::method => value.as_str().parse::<HttpMethod>(),
            _ => Err(anyhow!("The parsing result is not a valid `method`")),
        }
    }
}

/// converts `Pairs` into [`HttpVersion`]
impl TryFrom<Pair<'_, Rule>> for HttpVersion {
    type Error = anyhow::Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::version => value.as_str().parse::<HttpVersion>(),
            _ => Err(anyhow!("The parsing result is not a valid `version`")),
        }
    }
}

/// converts the body
impl TryFrom<Pair<'_, Rule>> for HttpBody {
    type Error = anyhow::Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::body => Ok(HttpBody::InlineText(value.as_str().to_owned())),
            _ => Err(anyhow!("The parsing result is not a valid `version`")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::http::HttpVersion::Http11;
    use crate::domain::http::*;

    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        indoc! {r#"
            ### GET gitignore template for rustlang
            GET https://api.github.com/gitignore/templates/Rust HTTP/1.1
            Accept: application/json
        "#},
        Bookmark {
            slug: "### GET gitignore template for rustlang".into(),
            request: HttpRequest {
                url: "https://api.github.com/gitignore/templates/Rust".into(),
                method: HttpMethod::Get,
                version: HttpVersion::Http11,
                headers: HttpHeaders::from(["Accept: application/json".to_owned()].as_slice()),
                body: HttpBody::default(),
                curl_params: Default::default(),
                placeholders: Default::default(),
            }
        }
    )]
    #[case(
        indoc! {r#"
            ### GET request with environment variables
            GET https://api.github.com/gitignore/templates/Rust HTTP/1.1
        "#},
        Bookmark {
            slug: "### GET request with environment variables".into(),
            request: HttpRequest {
                url: "https://api.github.com/gitignore/templates/Rust".into(),
                method: HttpMethod::Get,
                version: HttpVersion::Http11,
                headers: Default::default(),
                body: HttpBody::default(),
                curl_params: Default::default(),
                placeholders: Default::default(),
            }
        }
    )]
    #[case(
        indoc! {r#"
            ### this is a POST request with a body
            POST https://httpbin.org/anything HTTP/1.1
            Accept: application/json
            Content-Type: application/json

            {
                "foo": "Bar",
                "bool": true
            }
        "#},
        Bookmark {
            slug: "### this is a POST request with a body".into(),
            request: HttpRequest {
                url: "https://httpbin.org/anything".into(),
                method: HttpMethod::Post,
                version: HttpVersion::Http11,
                headers: HttpHeaders::from([
                    "Accept: application/json".to_owned(),
                    "Content-Type: application/json".to_owned(),
                ].as_slice()),
                body: HttpBody::InlineText(indoc! {r#"
                {
                    "foo": "Bar",
                    "bool": true
                }
                "#}.to_owned()),
                curl_params: Default::default(),
                placeholders: Default::default(),
            }
        }
    )]
    #[case(
        indoc! {r#"
            ### this is a POST request with a body
            POST https://httpbin.org/anything HTTP/1.1
            Accept : application/json
            Content-Type :application/json

            {
                "foo": "Bar",
                "bool": true
            }
        "#},
        Bookmark {
            slug: "### this is a POST request with a body".into(),
            request: HttpRequest {
                url: "https://httpbin.org/anything".into(),
                method: HttpMethod::Post,
                version: HttpVersion::Http11,
                headers: HttpHeaders::from([
                    "Accept: application/json".to_owned(),
                    "Content-Type: application/json".to_owned(),
                ].as_slice()),
                body: HttpBody::InlineText(indoc! {r#"
                {
                    "foo": "Bar",
                    "bool": true
                }
                "#}.to_owned()),
                curl_params: Default::default(),
                placeholders: Default::default(),
            }
        }
    )]
    fn should_parse_a_http_message(
        #[case] request_file_contents: &str,
        #[case] expected: Bookmark,
    ) {
        assert_eq!(
            parse_request_file(request_file_contents)
                .unwrap()
                .pop()
                .unwrap(),
            expected
        );
    }

    mod http_version {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_http_version_parsing() {
            let version = HttpParser::parse(Rule::version, "HTTP/1.1")
                .expect("parsing failed!")
                .next()
                .unwrap();
            let version: HttpVersion = version.try_into().unwrap();
            assert_eq!(version, Http11);
        }

        #[test]
        #[should_panic(expected = "Unsupported HTTP version: HTTP/1.0")]
        fn test_http_version_parsing_unsupported_version() {
            let _: HttpVersion = HttpParser::parse(Rule::version, "HTTP/1.0")
                .unwrap()
                .next()
                .unwrap()
                .try_into()
                .unwrap();
        }

        #[test]
        #[should_panic]
        fn test_http_version_parsing_parsing_error() {
            HttpParser::parse(Rule::version, "http/")
                .unwrap()
                .next()
                .unwrap();
        }
    }
}
