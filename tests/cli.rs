use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn no_jobs_valid_dir_lines() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("tests/data")
        .assert()
        .success()
        .stdout(predicate::str::contains("45"));

    Ok(())
}

#[test]
fn with_jobs_valid_dir_lines() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("-j2")
        .arg("tests/data")
        .assert()
        .success()
        .stdout(predicate::str::contains("45"));

    Ok(())
}

#[test]
fn no_jobs_valid_dir_words() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("-w")
        .arg("tests/data")
        .assert()
        .success()
        .stdout(predicate::str::contains("120"));

    Ok(())
}

#[test]
fn with_jobs_valid_dir_words() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("-wj2")
        .arg("tests/data")
        .assert()
        .success()
        .stdout(predicate::str::contains("120"));

    Ok(())
}

#[test]
fn no_jobs_multiple_path_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("tests/data/data.rs")
        .arg("tests/data/data.py")
        .arg("tests/data/data.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("45"));

    Ok(())
}

#[test]
fn no_jobs_invalid_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("fake_dir").assert().failure().stdout(
        predicate::str::contains("No such file or directory")
            .or(predicate::str::contains("cannot find the path specified")),
    );

    Ok(())
}

#[test]
fn invalid_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("--xxxyyyzzz")
        .arg("tests/data")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Found argument '--xxxyyyzzz' which wasn't expected",
        ));

    Ok(())
}

#[test]
fn get_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("--help").assert().success();
    cmd.arg("-h").assert().success();

    Ok(())
}

#[test]
fn get_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xloc")?;

    cmd.arg("--version").assert().success();
    cmd.arg("-V").assert().success();

    Ok(())
}
