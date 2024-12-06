mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::CMD;

#[test]
fn no_args() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn incorrect_arg() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.arg("--incorrect")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn missing_arg_value_short() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.arg("-n").assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("requires an argument")),
    );
}

#[test]
fn missing_arg_value_long() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.arg("--number-of-questions").assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("requires an argument")),
    );
}

#[test]
fn missing_command() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.arg("-d").assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("missing command")),
    );
}
