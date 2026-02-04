use rstest::rstest;
use std::{
  fs::File,
  path::{Path, PathBuf},
  process::Command,
};

fn test_cnf_files(group_name: &str, path: &Path) {
  insta::with_settings!({
    snapshot_path => format!("snapshots/{}", group_name),
    filters => [(r"(?sm)^Interactions\n.*", "[ Stats ]"),] },
  {
    let stdin = File::open(path).unwrap();
    let output = Command::new("vine")
      .args(["run", "sat/main.vi"])
      .args(["--lib", "sat"])
      .arg("--breadth-first")
      .stdin(stdin)
      .output()
      .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");
    let stderr = String::from_utf8(output.stderr).expect("Failed to convert stderr to string");

    if !output.status.success() {
      eprintln!("{stderr}");
      panic!("process exited unsuccessfully");
    }

    let name = path.file_stem().unwrap().to_string_lossy();
    insta::assert_snapshot!(format!("{name}_stdout"), stdout);
    insta::assert_snapshot!(format!("{name}_stderr"), stderr);
  });
}

#[rstest]
fn test_minimal(#[files("tests/minimal/*.cnf")] path: PathBuf) {
  test_cnf_files("minimal", &path);
}

#[rstest]
fn test_medium(#[files("tests/medium/*.cnf")] path: PathBuf) {
  test_cnf_files("medium", &path);
}

#[rstest]
fn test_satlib_sat(#[files("tests/uf5-218_first_10/*.cnf")] path: PathBuf) {
  test_cnf_files("uf5-218_first_10", &path);
}

#[rstest]
fn test_satlib_unsat(#[files("tests/uuf50-218_first_10/*.cnf")] path: PathBuf) {
  test_cnf_files("uuf50-218_first_10", &path);
}
