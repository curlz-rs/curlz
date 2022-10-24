use assert_cmd::prelude::*;
use dotenvy::dotenv;
use predicates::prelude::*;

fn binary() -> Result<std::process::Command, assert_cmd::cargo::CargoError> {
    std::process::Command::cargo_bin(env!("CARGO_PKG_NAME"))
}

#[test]
fn should_show_usage_when_no_args_passed() {
    binary()
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:"));
}

#[test]
fn should_request_a_url() {
    dotenv().ok();

    binary()
        .unwrap()
        .args(&["r", "https://api.github.com/gitignore/templates/Rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("# Generated by Cargo"));
}
