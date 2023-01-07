use assert_cmd::prelude::*;
use curlz::data::{HttpBody, HttpMethod};
use predicates::prelude::*;

use crate::testlib::{binary, CurlzTestSuite};

mod testlib;

#[test]
fn should_show_usage_when_no_args_passed() {
    binary()
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:"));
}

#[tokio::test]
async fn should_send_as_get() {
    CurlzTestSuite::new()
        .await
        .with_path("/gitignore/templates/Rust")
        .send_request()
        .await;
}

#[tokio::test]
async fn should_send_as_post() {
    CurlzTestSuite::new()
        .await
        .with_path("/anything")
        .with_method(HttpMethod::Post)
        .send_request()
        .await;
}

#[tokio::test]
async fn should_send_text_as_put() {
    CurlzTestSuite::new()
        .await
        .with_path("/anything")
        .with_method(HttpMethod::Put)
        .with_payload(HttpBody::InlineText("Howdy Pal!".to_string()))
        .send_request()
        .await;
}
