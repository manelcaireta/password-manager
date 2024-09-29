use assert_cmd::Command;

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
        .arg("OLD-PASSWORD-12345")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("update")
        .arg("TO-DELETE")
        .arg("NEW-PASSWORD-12345")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("get")
        .arg("TO-DELETE")
        .assert()
        .success()
        .stdout("TO-DELETE: NEW-PASSWORD-12345\n");

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("get")
        .arg("TO-DELETE")
        .arg("--version")
        .arg("1")
        .assert()
        .success()
        .stdout("TO-DELETE: OLD-PASSWORD-12345\n");

    let mut cmd = Command::cargo_bin("pwm")?;
    cmd.arg("rm")
        .arg("TO-DELETE")
        .write_stdin("YES")
        .assert()
        .success();

    Ok(())
}
