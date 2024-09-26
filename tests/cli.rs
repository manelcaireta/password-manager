use assert_cmd::{assert::OutputAssertExt, Command};

#[test]
fn create_and_update_password() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("rm")
        .arg("TO-DELETE")
        .write_stdin("YES\n")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("new")
        .arg("TO-DELETE")
        .arg("PASSWORD12345")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("get")
        .arg("TO-DELETE")
        .assert()
        .success()
        .stdout("TO-DELETE: PASSWORD12345\n");

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("update").arg("TO-DELETE").assert().success();

    let mut cmd = Command::cargo_bin("pwm")?;
    let output = cmd.arg("get").arg("TO-DELETE").output()?;
    output.clone().assert().success();
    assert_ne!("TO-DELETE: PASSWORD12345\n", String::from_utf8(output.stdout)?);

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("rm")
        .arg("TO-DELETE")
        .write_stdin("YES")
        .assert()
        .success();

    Ok(())
}
