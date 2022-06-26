use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn should_show_usage_when_no_args_passed() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:"));
}
