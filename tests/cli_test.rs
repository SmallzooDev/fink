use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn should_display_version_when_version_flag_provided() {
    let mut cmd = Command::cargo_bin("jkms").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("jkms 0.1.0"));
}
