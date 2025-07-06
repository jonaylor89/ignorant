use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("ignorant").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Check if a phone number is used on different sites"))
        .stdout(predicate::str::contains("COUNTRY_CODE"))
        .stdout(predicate::str::contains("PHONE"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("ignorant").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1.2.0"));
}

#[test]
fn test_cli_missing_args() {
    let mut cmd = Command::cargo_bin("ignorant").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"));
}

#[test]
fn test_cli_invalid_timeout() {
    let mut cmd = Command::cargo_bin("ignorant").unwrap();
    cmd.args(["33", "123456789", "--timeout", "abc"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test] 
fn test_cli_with_flags() {
    let mut cmd = Command::cargo_bin("ignorant").unwrap();
    cmd.args(["33", "123456789", "--no-color", "--no-clear", "--only-used", "--timeout", "1"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("+33 123456789"))
        .stdout(predicate::str::contains("3 websites checked"));
}

#[test]
fn test_cli_short_timeout() {
    let mut cmd = Command::cargo_bin("ignorant").unwrap();
    cmd.args(["1", "5551234567", "-T", "1", "--no-clear"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("+1 5551234567"));
}

#[test]
fn test_cli_output_format() {
    let mut cmd = Command::cargo_bin("ignorant").unwrap();
    cmd.args(["44", "7700900000", "--no-clear", "--timeout", "2"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Twitter : @palenath"))
        .stdout(predicate::str::contains("Github : https://github.com/megadose/ignorant"))
        .stdout(predicate::str::contains("For BTC Donations"))
        .stdout(predicate::str::contains("*"))
        .stdout(predicate::str::contains("+44 7700900000"));
}