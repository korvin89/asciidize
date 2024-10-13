use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_print() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("asciidize")?;
    cmd.arg("alphabet");
    cmd.arg("print");
    cmd.arg("--width").arg("3");
    cmd.arg("--alphabet").arg("abcdefg");

    let expected_stdout = "       \n XXXXX \n XabcX \n XdefX \n Xg  X \n XXXXX \n       \n";

    cmd.assert().success();
    cmd.assert().code(0);
    cmd.assert().stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_print_default_alphabet() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("asciidize")?;
    cmd.arg("alphabet");
    cmd.arg("print");

    cmd.assert().success();
    cmd.assert().code(0);

    Ok(())
}
