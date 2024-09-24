use std::{fs::File, io::{Read, Write}};

pub fn run_checks () {
  // We don't have any checks to run for Kotlin.
}

fn open_build_gradle_kts () -> File {
  File::open("library/build.gradle.kts")
    .unwrap()
}

/// Reads the `library/build.gradle.kts` file and parses it as KTS
/// and returns the value of the `version` property as string.
pub fn get_current_version () -> String {
  let mut file = open_build_gradle_kts();

  let mut buffer = String::new();
  file.read_to_string(&mut buffer).expect("file should be proper KTS");

  // Find the `version = "x.y.z"` line.
  let version_line = buffer.lines().find(|line| line.contains("version"))
    .expect("'build.gradle.kts' is missing 'version' property.");

  let version = version_line.split("\"")
    .nth(1)
    .expect("'version' should be wrapped in double quotes.");

  version.to_string()
}

pub fn bump_version (version: &str) {
  let mut file = open_build_gradle_kts();

  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  let from = format!("version = \"{}\"", get_current_version());
  let to = format!("version = \"{}\"", version);

  let content = content.replace(&from, &to);

  let mut file = File::create("library/build.gradle.kts").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}
