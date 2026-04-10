use assert_cmd::Command;
use predicates::str::contains;

const VALID_FILE_PATH: &str = "src/data.json";

#[test]
fn test_invalid_index() {
    let mut cmd = Command::cargo_bin("jq-lite").unwrap();

    cmd.arg("user[abc]")
        .arg(VALID_FILE_PATH)
        .assert()
        .failure()
        .code(2)
        .stderr(contains("Invalid array index"));
}

#[test]
fn test_invalid_tokens() {
    let mut cmd = Command::cargo_bin("jq-lite").unwrap();

    cmd.arg("user[")
        .arg(VALID_FILE_PATH)
        .assert()
        .failure()
        .code(2)
        .stderr(contains("Unclosed '[' in query"));
}

#[test]
fn test_invalid_token_sequence() {
    let mut cmd = Command::cargo_bin("jq-lite").unwrap();

    cmd.arg("[].user.address[]")
        .arg(VALID_FILE_PATH)
        .assert()
        .failure()
        .code(2)
        .stderr(contains("Query cannot start with array access"));
}

#[test]
fn test_valid_query_execution() {
    let mut cmd = Command::cargo_bin("jq-lite").unwrap();

    cmd.arg("user.name")
        .arg(VALID_FILE_PATH)
        .assert()
        .success()
        .code(0)
        .stdout(contains("Ada"));
}

#[test]
fn test_valid_nested_query_execution() {
    let mut cmd = Command::cargo_bin("jq-lite").unwrap();

    cmd.arg("user.address[0].city[0]")
        .arg(VALID_FILE_PATH)
        .assert()
        .success()
        .code(0)
        .stdout(contains("name"))
        .stdout(contains("zip"));
}
