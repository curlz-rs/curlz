use assert_cmd::prelude::*;
use curlz::request::http::{HttpBody, HttpMethod};
use predicates::prelude::*;
use predicates::str::contains;

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
        .with_path("/gitignore/templates/Rust")
        .send_request()
        .await;
}

#[tokio::test]
async fn should_send_as_post() {
    CurlzTestSuite::new()
        .with_path("/post")
        .with_method(HttpMethod::Post)
        .send_request()
        .await;
}

#[tokio::test]
async fn should_send_text_as_put() {
    CurlzTestSuite::new()
        .with_path("/put")
        .with_method(HttpMethod::Put)
        .with_payload(HttpBody::InlineText("Howdy Pal!".to_string()))
        .send_request()
        .await;
}

#[tokio::test]
async fn should_send_as_post_with_body_variables() {
    CurlzTestSuite::new()
        .with_env_variable("id", "1")
        .with_env_variable("username", "john")
        .with_path("/post")
        .with_method(HttpMethod::Post)
        .with_payload(r#"{ "id": {{ id }}, "user": "{{ username }}" }"#)
        .expect_payload(contains(r#"{ "id": 1, "user": "john" }"#))
        .send_request()
        .await;
}
