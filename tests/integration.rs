use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::{fs, io::Write};
use tempfile::NamedTempFile;

#[cfg(test)]
fn setup_command(file: &mut NamedTempFile, version: &str, command: Vec<&str>) -> (Command, String) {
    writeln!(
        file,
        "[package]\nversion = \"{}\"\n\n[dependencies]\nversion = \"{}\"",
        version, version,
    )
    .unwrap();
    let path = file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(command);
    cmd.arg("--config").arg(path);

    (cmd, path.to_string())
}

#[test]
fn get() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec![]);
    cmd.assert().success().stdout("1.0.0\n");

    let contains = predicate::str::contains("1.0.0");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}

#[test]
fn patch() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["patch"]);
    cmd.assert().success().stdout("1.0.1\n");

    let contains = predicate::str::contains("1.0.1");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}

#[test]
fn minor() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["minor"]);
    cmd.assert().success().stdout("1.1.0\n");

    let contains = predicate::str::contains("1.1.0");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}

#[test]
fn major() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["major"]);
    cmd.assert().success().stdout("2.0.0\n");

    let contains = predicate::str::contains("2.0.0");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}

#[test]
fn major_pre() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["major", "--pre", "alpha"]);
    cmd.assert().success().stdout("2.0.0-alpha.1\n");

    let contains = predicate::str::contains("2.0.0-alpha.1");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}

#[test]
fn pre() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["pre", "alpha"]);
    cmd.assert().success().stdout("1.0.0-alpha.1\n");

    let contains = predicate::str::contains("1.0.0-alpha.1");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));

    // run again without `alpha`
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["pre"]);
    cmd.assert().success().stdout("1.0.0-alpha.2\n");

    let contains = predicate::str::contains("1.0.0-alpha.2");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));

    // change to `beta`
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["pre", "beta"]);
    cmd.assert().success().stdout("1.0.0-beta.1\n");

    let contains = predicate::str::contains("1.0.0-beta.1");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}

#[test]
fn keep_dependency_version() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["major"]);
    cmd.assert().success().stdout("2.0.0\n");

    let contains = predicate::str::contains("version = \"2.0.0\"");
    let contains_dep = predicate::str::contains("version = \"1.0.0\"");
    let content = &fs::read_to_string(&path).unwrap();
    assert_eq!(true, contains.eval(&content));
    assert_eq!(true, contains_dep.eval(&content));
}

#[test]
fn bad_input() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["set", "1.0"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("expected more input"));
    let contains = predicate::str::contains("1.0.0");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}

#[test]
fn missing_pre_version() {
    let mut file = NamedTempFile::new().unwrap();
    let (mut cmd, path) = setup_command(&mut file, "1.0.0", vec!["pre"]);
    cmd.assert().failure().stderr(predicate::str::contains(
        "run `cargo-semver pre [alpha|beta]` first to add a new pre-release version.",
    ));
    let contains = predicate::str::contains("1.0.0");
    assert_eq!(true, contains.eval(&fs::read_to_string(path).unwrap()));
}
