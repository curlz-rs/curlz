use assert_cmd::assert::Assert;
use assert_cmd::prelude::*;
use dotenvy::dotenv;
use predicates::prelude::*;
use std::process::Command;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn binary() -> Result<Command, assert_cmd::cargo::CargoError> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
}

#[test]
fn should_show_usage_when_no_args_passed() {
    binary()
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:"));
}

#[tokio::test]
async fn should_request_a_url() {
    CurlzTest::new()
        .await
        .with_url("/gitignore/templates/Rust")
        .request()
        .await;
}

struct CurlzTest {
    mock_server: MockServer,
    url_part: String,
    payload: String,
}

impl CurlzTest {
    pub async fn new() -> Self {
        dotenv().ok();
        Self {
            mock_server: MockServer::start().await,
            url_part: "".to_string(),
            payload: "# curlz rocks".to_string(),
        }
    }

    /// sets the target url to be requested
    pub fn with_url(mut self, url_part: &str) -> Self {
        self.url_part = url_part.to_string();
        self
    }

    /// runs curlz and requests the given url
    pub async fn request(self) -> Assert {
        Mock::given(method("GET"))
            .and(path(self.url_part.as_str()))
            .respond_with(ResponseTemplate::new(200).set_body_string(&self.payload))
            .mount(&self.mock_server)
            .await;

        let url = format!("{}{}", &self.mock_server.uri(), self.url_part);
        binary()
            .unwrap()
            .args(["r", url.as_str()])
            .assert()
            .success()
            .stdout(predicate::str::contains(&self.payload))
    }
}
