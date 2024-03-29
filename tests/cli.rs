use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

// Because this file is located in `./tests/cli.rs`, it will be found by `$ cargo test` and run automatically

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("foobar").arg("test/file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\nAnother test\nnext is a blank line\n\nsome more text\ntest 3"
    )?;

    let mut cmd = Command::main_binary()?;
    cmd.arg("test").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test\ntest 3"));

    Ok(())
}

#[test]
fn find_no_results() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nAnother test\nSome text")?;

    let mut cmd = Command::main_binary()?;
    cmd.arg("pizza").arg(file.path());
    cmd.assert().success().stdout(predicate::eq(b"" as &[u8]));
    Ok(())
}
