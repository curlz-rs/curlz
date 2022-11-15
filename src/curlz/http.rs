//! a module for experimenting with the http language that the rest client uses
use crate::data::{Bookmark, HttpHeaders, HttpMethod, HttpRequest, HttpVersion};
use anyhow::anyhow;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/http-grammar.pest"] // relative to project `src`
struct HttpParser;

#[allow(dead_code)]
fn parse_request_file(req_file: impl AsRef<str>) -> Vec<Bookmark> {
    let mut requests = vec![];

    let req_file = req_file.as_ref();
    let file = HttpParser::parse(Rule::file, req_file)
        .expect("parsing failed!")
        .next()
        .unwrap();

    let mut slug: String = "".to_owned();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::request => {
                requests.push(Bookmark {
                    slug: slug.to_owned(),
                    request: HttpRequest::from(line),
                });
            }
            Rule::delimiter => {
                slug = line.as_str().trim().to_owned();
                println!("delimiter = {:?}", line.as_str());
            }
            x => {
                println!("x = {:?}\n", x);
            }
        }
    }

    requests
}

/// todo: write tests
impl From<Pair<'_, Rule>> for HttpHeaders {
    fn from(headers: Pair<'_, Rule>) -> Self {
        match headers.as_rule() {
            Rule::headers => {
                let mut h: HttpHeaders = Default::default();
                for header in headers.into_inner() {
                    let mut inner_rules = header.into_inner();

                    let name: &str = inner_rules.next().unwrap().as_str();
                    let value: &str = inner_rules.next().unwrap().as_str();

                    h.push(name, value);
                }
                h
            }
            _ => unreachable!("this should not happen"),
        }
    }
}

/// todo: write tests
impl From<Pair<'_, Rule>> for HttpRequest {
    fn from(request: Pair<'_, Rule>) -> Self {
        match request.as_rule() {
            Rule::request => {
                let mut inner_rules = request.into_inner();
                let method: HttpMethod = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<HttpMethod>()
                    .unwrap();
                let url = inner_rules.next().unwrap().as_str().to_string();
                let version: HttpVersion = inner_rules.next().unwrap().try_into().unwrap();
                let headers = inner_rules.next();

                dbg!(&method);
                dbg!(&url);
                dbg!(&version);
                // dbg!(body);

                Self {
                    url,
                    method,
                    version,
                    headers: headers.map(HttpHeaders::from).unwrap_or_default(),
                    curl_params: Default::default(),
                    placeholders: Default::default(),
                }
            }
            _ => {
                unreachable!("the client code ensures this cannot happen!")
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::*;

    use crate::data::HttpVersion::Http11;
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
            parse_request_file(request_file_contents).pop().unwrap(),
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
