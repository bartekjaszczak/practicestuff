mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::CMD;

#[test]
fn times_table_unrecognised_arg() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(["times_table", "--incorrect"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Usage:").and(predicate::str::contains("unrecognised option")),
        );
}

#[test]
fn times_table_missing_arg_value_long() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(["times_table", "--upper-boundary-1"]).assert().failure().stderr(
        predicate::str::contains("Usage:").and(predicate::str::contains("requires an argument")),
    );
}

#[test]
fn times_table_show_help() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    let pred = predicate::str::contains("Usage:");
    let pred = pred.and(predicate::str::contains("Times table options"));
    let pred = pred.and(predicate::str::contains("Display help for times_table command"));

    cmd.arg("times_table").arg("-h").assert().success().stdout(pred);
}

#[test]
fn times_table_one_question_correct_answer() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args([
        "--number-of-questions=1",
        "times_table",
        "--lower-boundary-1=7",
        "--upper-boundary-1=7",
        "--lower-boundary-2=7",
        "--upper-boundary-2=7",
    ]) // Essentially guarantees question 7*7 (=49)
    .write_stdin("49")
    .assert()
    .success()
    .stdout(predicate::str::contains("Correct answers: 1/1"));
}

#[test]
fn times_table_one_question_incorrect_answer() {
    let mut cmd = Command::cargo_bin(CMD).expect("crate not found");
    cmd.args(["--number-of-questions=1", "times_table"])
        .write_stdin("hehe") // "hehe" is probably not an answer to any times
                                                  // table question
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Correct answers: 0/1")
                .and(predicate::str::contains("Correct answer:")),
        );
}
