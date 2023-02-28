use assert_cmd::prelude::*;
use std::collections::HashMap;
use std::ops::Deref;

use assert_cmd::assert::Assert;
use dotenvy::dotenv;
use predicates::str::contains;
use predicates::{BoxPredicate, Predicate};
use std::process::Command;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

use curlz::domain::http::{HttpBody, HttpMethod};

pub fn binary() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

pub struct CurlzTestSuite {
    url_part: String,
    http_method: String,
    payload: HttpBody,
    defined_variables: HashMap<String, String>,
    expected_stdout: BoxPredicate<str>,
}

impl Default for CurlzTestSuite {
    fn default() -> Self {
        dotenv().ok();
        Self {
            url_part: "/".to_string(),
            http_method: "GET".to_string(),
            payload: HttpBody::None,
            defined_variables: Default::default(),
            expected_stdout: BoxPredicate::new(contains("")),
        }
    }
}

impl CurlzTestSuite {
    pub fn new() -> Self {
        Self::default()
    }

    /// runs curlz and requests the given url from a local echo http server
    pub async fn send_request(mut self) -> Assert {
        let mock_server = self.prepare_mock_server().await;

        binary()
            .arg("r")
            .args(self.args_method())
            .args(self.args_define())
            .args(self.args_data())
            .arg(self.arg_url(&mock_server))
            .assert()
            .success()
            .stdout(self.expected_stdout)
            .stderr(contains("% Total"))
    }

    /// prepares a variable that ends as a `--define name=value` cli argument
    pub fn with_env_variable(mut self, name: &str, value: &str) -> Self {
        self.defined_variables
            .insert(name.to_string(), value.to_string());
        self
    }

    /// sets the target url to be requested
    pub fn with_path(mut self, url_part: &str) -> Self {
        self.url_part = url_part.to_string();
        self
    }

    /// sets the http method used for the request
    pub fn with_method(mut self, http_method: HttpMethod) -> Self {
        self.http_method = (&http_method).into();
        self
    }

    /// sets the http body payload that is send
    /// also readjusts the expected output based on the given payload
    pub fn with_payload(mut self, body: impl Into<HttpBody>) -> Self {
        self.payload = body.into();
        let predicate = {
            let pattern = String::from_utf8_lossy(self.payload.as_bytes().unwrap());
            contains(pattern.deref())
        };

        self.expect_payload(predicate)
    }

    /// sets the expected output
    pub fn expect_payload<P: Predicate<str>>(mut self, predicate: P) -> Self
    where
        P: Send + Sync + 'static,
    {
        self.expected_stdout = BoxPredicate::new(predicate);
        self
    }

    fn args_method(&self) -> [&str; 2] {
        ["-X", self.http_method.as_str()]
    }

    fn arg_url(&self, mock_server: &MockServer) -> String {
        format!("{}{}", mock_server.uri(), self.url_part)
    }

    fn args_data(&self) -> Vec<&str> {
        match &self.payload {
            HttpBody::None => vec![],
            HttpBody::InlineText(s) => vec!["-d", s.as_str()],
            HttpBody::InlineBinary(_) => {
                todo!("binary data are not yet supported for the http body")
            }
            HttpBody::Extern(_) => {
                todo!("external file references are not yet supported for the http body")
            }
        }
    }

    fn args_define(&self) -> Vec<String> {
        self.defined_variables
            .iter()
            .flat_map(|(name, value)| vec!["--define".to_string(), format!("{name}={value}")])
            .collect::<Vec<_>>()
    }

    async fn prepare_mock_server(&mut self) -> MockServer {
        let mock_server = MockServer::start().await;

        Mock::given(method(self.http_method.as_str()))
            .and(path(self.url_part.as_str()))
            .respond_with(EchoResponder::default())
            .mount(&mock_server)
            .await;

        mock_server
    }
}

#[derive(Default)]
struct EchoResponder;

impl Respond for EchoResponder {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_bytes(request.body.as_slice())
    }
}
