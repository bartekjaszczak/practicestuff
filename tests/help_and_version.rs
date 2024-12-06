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
fn show_help_short() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    let pred = predicate::str::contains("Usage:");
    let pred = pred.and(predicate::str::contains("General options"));
    let pred = pred.and(predicate::str::contains("Commands"));
    let pred = pred.and(predicate::str::contains("Display this help message"));

    cmd.arg("-h").assert().success().stdout(pred);
}

#[test]
fn show_help_long() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    let pred = predicate::str::contains("Usage:");
    let pred = pred.and(predicate::str::contains("General options"));
    let pred = pred.and(predicate::str::contains("Commands"));
    let pred = pred.and(predicate::str::contains("Display this help message"));

    cmd.arg("--help").assert().success().stdout(pred);
}

#[test]
fn show_version() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
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
fn missing_command() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.arg("-d").assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("missing command")),
    );
}

#[test]
fn powers_show_help() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    let pred = predicate::str::contains("Usage:");
    let pred = pred.and(predicate::str::contains("Powers options"));
    let pred = pred.and(predicate::str::contains("Display help for powers command"));

    cmd.arg("powers").arg("-h").assert().success().stdout(pred);
}

#[test]
fn powers_one_question() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args([
        "--number-of-questions=1",
        "powers",
        "-b",
        "7",
        "-l",
        "2",
        "-u",
        "2",
    ]) // Essentially guarantees question 7^2 (= 49)
    .write_stdin("49")
    .assert()
    .success()
    .stdout(predicate::str::contains("Correct answers: 1/1"));
}
