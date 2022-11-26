use assert_cmd::prelude::*;
use predicates::prelude::*;

use assert_cmd::assert::Assert;
use dotenvy::dotenv;
use std::process::Command;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use curlz::data::HttpMethod;

pub fn binary() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

pub struct CurlzTestSuite {
    mock_server: MockServer,
    url_part: String,
    http_method: String,
    payload: String,
}

impl CurlzTestSuite {
    pub async fn new() -> Self {
        dotenv().ok();
        Self {
            mock_server: MockServer::start().await,
            url_part: "/".to_string(),
            http_method: "GET".to_string(),
            payload: "# curlz rocks".to_string(),
        }
    }

    /// sets the target url to be requested
    pub fn send_to_path(mut self, url_part: &str) -> Self {
        self.url_part = url_part.to_string();
        self
    }

    /// sets the http method used for the request
    pub fn send_with_method(mut self, http_method: HttpMethod) -> Self {
        self.http_method = (&http_method).into();
        self
    }

    /// runs curlz and requests the given url
    pub async fn issue_request(&mut self) -> Assert {
        self.prepare_mock_server().await;

        let url = format!("{}{}", &self.mock_server.uri(), self.url_part);
        binary()
            .arg("r")
            .args(["-X", self.http_method.as_str()])
            .arg(url.as_str())
            .assert()
            .success()
            .stdout(predicate::str::contains(&self.payload))
            .stderr(predicate::str::contains("% Total"))
    }

    async fn prepare_mock_server(&mut self) {
        Mock::given(method(self.http_method.as_str()))
            .and(path(self.url_part.as_str()))
            .respond_with(ResponseTemplate::new(200).set_body_string(self.payload.as_str()))
            .mount(&self.mock_server)
            .await;
    }
}
