use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn q1() -> TestResult {
    let expected = "Ascending order: [1, 2, 3, 4, 5]\nDescending order: [5, 4, 3, 2, 1]\n";
    let mut cmd = Command::cargo_bin("hw7")?;
    cmd.arg("5").arg("1").arg("3").arg("4").arg("2").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn q1_2() -> TestResult {
    let expected = "Ascending order: [1, 2, 3, 4, 5]\nDescending order: [5, 4, 3, 2, 1]\n";
    let mut cmd = Command::cargo_bin("1-2")?;
    cmd.arg("5").arg("1").arg("3").arg("4").arg("2").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn q2_1() -> TestResult {
    let expected = "Ascending order by X: [(1.0, 5.0), (2.0, 7.0)]\nAscending order by Y: [(1.0, 5.0), (2.0, 7.0)]\nDescending order by X: [(2.0, 7.0), (1.0, 5.0)]\nDescending order by Y: [(2.0, 7.0), (1.0, 5.0)]\n";
    let mut cmd = Command::cargo_bin("2-1")?;
    cmd.arg("1").arg("5").arg("2").arg("7").arg("3").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn q2_2() -> TestResult {
    let expected = "Ascending order by X: [(1.0, 5.0), (2.0, 7.0)]\nAscending order by Y: [(1.0, 5.0), (2.0, 7.0)]\nDescending order by X: [(2.0, 5.0), (1.0, 7.0)]\nDescending order by Y: [(2.0, 7.0), (1.0, 5.0)]\n";
    let mut cmd = Command::cargo_bin("2-2")?;
    cmd.arg("1").arg("5").arg("2").arg("7").arg("3").assert().success().stdout(expected);

    Ok(())
}