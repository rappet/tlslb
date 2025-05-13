use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

const CMD_NAME: &str = "tlslb";

#[test]
fn fails_without_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CMD_NAME)?;

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:",
    ));

    Ok(())
}
