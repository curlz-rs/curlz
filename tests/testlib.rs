use assert_cmd::prelude::*;
use predicates::prelude::*;

use assert_cmd::assert::Assert;
use dotenvy::dotenv;
use std::process::Command;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

use curlz::data::{HttpBody, HttpMethod};

pub fn binary() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

pub struct CurlzTestSuite {
    mock_server: MockServer,
    url_part: String,
    http_method: String,
    payload: HttpBody,
}

impl CurlzTestSuite {
    pub async fn new() -> Self {
        dotenv().ok();
        Self {
            mock_server: MockServer::start().await,
            url_part: "/".to_string(),
            http_method: "GET".to_string(),
            payload: HttpBody::None,
        }
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
    pub fn with_payload(mut self, body: HttpBody) -> Self {
        self.payload = body;
        self
    }

    /// runs curlz and requests the given url from a local echo http server
    pub async fn send_request(&mut self) -> Assert {
        self.prepare_mock_server().await;

        let url = format!("{}{}", &self.mock_server.uri(), self.url_part);
        // todo: if body is binary, this will not work, better not have expectations for binary payloads
        let expected_stdout =
            predicate::str::contains(String::from_utf8_lossy(self.payload.as_bytes().unwrap()));
        let data = match &self.payload {
            HttpBody::None => vec![],
            HttpBody::InlineText(s) => vec!["-d", s.as_str()],
            _ => todo!(),
        };

        binary()
            .arg("r")
            .args(["-X", self.http_method.as_str()])
            .args(data)
            .arg(url.as_str())
            .assert()
            .success()
            .stdout(expected_stdout)
            .stderr(predicate::str::contains("% Total"))
    }

    async fn prepare_mock_server(&mut self) {
        Mock::given(method(self.http_method.as_str()))
            .and(path(self.url_part.as_str()))
            .respond_with(EchoResponder::default())
            .mount(&self.mock_server)
            .await;
    }
}

#[derive(Default)]
struct EchoResponder;

impl Respond for EchoResponder {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_bytes(request.body.as_slice())
    }
}
