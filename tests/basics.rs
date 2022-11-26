use assert_cmd::prelude::*;
use curlz::data::HttpMethod;
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
        .send_to_path("/gitignore/templates/Rust")
        .issue_request()
        .await;
}

#[tokio::test]
async fn should_send_payload_as_post() {
    CurlzTestSuite::new()
        .await
        .send_to_path("/anything")
        .send_with_method(HttpMethod::Post)
        .issue_request()
        .await;
}
