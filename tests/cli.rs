use assert_cmd::Command;

#[test]
fn translation() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("unitr")?;
    cmd.arg("老海")
        .arg("海鲜")
        .write_stdin("老人与海")
        .assert()
        .success()
        .code(0)
        .stdout("海人与鲜");
    Ok(())
}

#[test]
fn complement() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("unitr")?;
    cmd.arg("-c")
        .arg("三元")
        .arg("拼")
        .write_stdin("三份归元气")
        .assert()
        .success()
        .code(0)
        .stdout("三拼拼元拼");
    Ok(())
}

#[test]
fn squeezing() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("unitr")?
        .arg("-s")
        .arg("朝暮")
        .write_stdin("朝朝暮暮")
        .assert()
        .success()
        .code(0)
        .stdout("朝暮");
    Command::cargo_bin("unitr")?
        .arg("-s")
        .arg("朝")
        .write_stdin("朝朝暮暮")
        .assert()
        .success()
        .code(0)
        .stdout("朝暮暮");
    Command::cargo_bin("unitr")?
        .arg("-s")
        .arg("暮")
        .write_stdin("朝朝暮暮")
        .assert()
        .success()
        .code(0)
        .stdout("朝朝暮");
    Ok(())
}

#[test]
fn deleting() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("unitr")?
        .arg("-d")
        .arg("十二")
        .write_stdin("四十二加十一")
        .assert()
        .success()
        .code(0)
        .stdout("四加一");
    Ok(())
}
