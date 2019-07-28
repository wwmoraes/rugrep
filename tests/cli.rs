use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

#[test]
fn no_args() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("rugrep")?;
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: not enough parameters"));

    Ok(())
}

#[test]
fn insufficient_args() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("rugrep")?;
    
    cmd.arg("something");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: not enough parameters"));

    Ok(())
}

#[test]
fn file_not_exists() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("rugrep")?;
    
    cmd.arg("something").arg("non-existent-file");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: No such file or directory (os error 2)"));

    Ok(())
}

#[test]
fn no_content_found() -> Result<(), Box<std::error::Error>> {
    // Prepare the test file
    std::fs::write("haystack-hay", "hay\nhay\nhay")?;

    let mut cmd = Command::cargo_bin("rugrep")?;
    
    cmd.arg("needle").arg("haystack-hay");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(""));

    // Cleanup
    std::fs::remove_file("haystack-hay")?;

    Ok(())
}

#[test]
fn content_found() -> Result<(), Box<std::error::Error>> {
    // Prepare the test file
    std::fs::write("haystack-needle", "hay\nneedle\nhay")?;

    let mut cmd = Command::cargo_bin("rugrep")?;
    
    cmd.arg("needle").arg("haystack-needle");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("haystack-needle at 2: needle"));

    // Cleanup
    std::fs::remove_file("haystack-needle")?;

    Ok(())
}