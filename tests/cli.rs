use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn create_and_update_password() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pwm")?;

    cmd.arg("new").arg("TO-DELETE");
    cmd.arg("get").arg("TO-DELETE");
    cmd.arg("update").arg("TO-DELETE");
    cmd.arg("get").arg("TO-DELETE");
    cmd.arg("delete").arg("TO-DELETE");

    Ok(())
}
