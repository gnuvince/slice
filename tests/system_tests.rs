use std::process::Command;

#[test]
fn two_offsets() {
    let out = Command::new(env!("CARGO_BIN_EXE_slice"))
        .arg("tests/test00")
        .arg("4")
        .arg("8")
        .output()
        .unwrap();
    assert_eq!(out.stdout, b"efgh");
}

#[test]
fn one_offsets() {
    let out = Command::new(env!("CARGO_BIN_EXE_slice"))
        .arg("tests/test00")
        .arg("4")
        .output()
        .unwrap();
    assert_eq!(out.stdout, b"efghijklmnopqrstuvwxyz\n");
}

#[test]
fn zero_offsets() {
    let out = Command::new(env!("CARGO_BIN_EXE_slice"))
        .arg("tests/test00")
        .output()
        .unwrap();
    assert_eq!(out.stdout, b"abcdefghijklmnopqrstuvwxyz\n");
}

#[test]
fn hex_offsets() {
    let out = Command::new(env!("CARGO_BIN_EXE_slice"))
        .arg("tests/test00")
        .arg("0xa")
        .arg("0X0C")
        .output()
        .unwrap();
    assert_eq!(out.stdout, b"kl");
}
