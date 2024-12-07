mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::CMD;

#[test]
fn doomsday_unrecognised_arg() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(["doomsday", "--incorrect"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Usage:").and(predicate::str::contains("unrecognised option")),
        );
}

#[test]
fn doomsday_missing_arg_value_short() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(["doomsday", "-l"]).assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("requires an argument")),
    );
}

#[test]
fn doomsday_missing_arg_value_long() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(["doomsday", "--lower-boundary"]).assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("requires an argument")),
    );
}

#[test]
fn doomsday_show_help() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    let pred = predicate::str::contains("Usage:");
    let pred = pred.and(predicate::str::contains("Doomsday options"));
    let pred = pred.and(predicate::str::contains("Display help for doomsday command"));

    cmd.arg("doomsday").arg("-h").assert().success().stdout(pred);
}

#[test]
fn doomsday_one_question_incorrect_answer() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(["--number-of-questions=1", "doomsday"])
        .write_stdin("hehe") // "hehe" is not a weekday
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Correct answers: 0/1")
                .and(predicate::str::contains("Correct answer:")),
        );
}
