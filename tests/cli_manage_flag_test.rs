use assert_cmd::Command;

#[test]
fn should_accept_manage_flag_with_help() {
    let mut cmd = Command::cargo_bin("fink").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("--manage"))
        .stdout(predicates::str::contains("-m"))
        .stdout(predicates::str::contains("Enter management mode"));
}

#[test]
fn should_not_conflict_with_subcommands() {
    use tempfile::tempdir;
    
    let temp_dir = tempdir().unwrap();
    
    // Should be able to use list command without manage flag
    let mut cmd = Command::cargo_bin("fink").unwrap();
    cmd.arg("list")
        .arg("--path")
        .arg(temp_dir.path())
        .assert()
        .success();
}