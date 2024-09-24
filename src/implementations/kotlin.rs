use std::{fs::File, io::{Read, Write}};

pub fn run_checks () {
  // We don't have any checks to run for Kotlin.
}

fn read_build_gradle_kts () -> String {
  let mut file = File::open("library/build.gradle.kts")
    .unwrap();

  let mut buffer = String::new();
  file.read_to_string(&mut buffer).unwrap();

  buffer
}

/// Reads the `library/build.gradle.kts` file and parses it as KTS
/// and returns the value of the `version` property as string.
pub fn get_current_version () -> String {
  let content = read_build_gradle_kts();

  // Find the `version = "x.y.z"` line.
  let version_line = content.lines().find(|line| line.contains("version"))
    .expect("'build.gradle.kts' is missing 'version' property.");

  let version = version_line.split("\"")
    .nth(1)
    .expect("'version' should be wrapped in double quotes.");

  version.to_string()
}

pub fn bump_version (version: &str) {
  let content = read_build_gradle_kts();

  let from = format!("version = \"{}\"", get_current_version());
  let to = format!("version = \"{}\"", version);

  let content = content.replace(&from, &to);

  let mut file = File::create("library/build.gradle.kts").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}
