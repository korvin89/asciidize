use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_none() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("asciidize")?;

    cmd.assert().failure();
    cmd.assert().code(64);
    cmd.assert().stderr("No command provided\n");

    Ok(())
}
