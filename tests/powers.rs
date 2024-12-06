mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::CMD;

#[test]
fn powers_unrecognised_arg() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(&["powers", "--incorrect"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Usage:").and(predicate::str::contains("unrecognised option")),
        );
}

#[test]
fn powers_missing_arg_value_short() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(&["powers", "-b"]).assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("requires an argument")),
    );
}

#[test]
fn powers_missing_arg_value_long() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(&["powers", "--base"]).assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("requires an argument")),
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
fn powers_one_question_correct_answer() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(&[
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

#[test]
fn powers_one_question_incorrect_answer() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(&["--number-of-questions=1", "powers"])
        .write_stdin("hehe") // "hehe" is probably not a power of 2
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Correct answers: 0/1")
                .and(predicate::str::contains("Correct answer:")),
        );
}
