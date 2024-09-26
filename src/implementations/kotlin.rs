use std::{fs::File, io::{Read, Write}};
use crate::utils::find_between;

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

fn read_readme () -> String {
  let mut file = File::open("README.md")
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

fn bump_build_gradle_kts (version: &str) {
  let content = read_build_gradle_kts();

  let from = format!("version = \"{}\"", get_current_version());
  let to = format!("version = \"{}\"", version);

  let content = content.replace(&from, &to);

  let mut file = File::create("library/build.gradle.kts").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

fn bump_readme (version: &str) {
  let content = read_readme();
  let artifact_id = find_between(&content, "<artifactId>", "</artifactId>");

  // implementation 'ink.literate:lib_name:version'

  // replace for maven section
  let from = format!("<version>{}</version>", get_current_version());
  let to = format!("<version>{}</version>", version);
  let content = content.replace(&from, &to);
  
  // replace for gradle (kotlin) section
  let from = format!("implementation(\"ink.literate:{artifact_id}:{}\")", get_current_version());
  let to = format!("implementation(\"ink.literate:{artifact_id}:{}\")", version);
  let content = content.replace(&from, &to);

  // replace for gradle section
  let from = format!("implementation 'ink.literate:{artifact_id}:{}'", get_current_version());
  let to = format!("implementation 'ink.literate:{artifact_id}:{}'", version);
  let content = content.replace(&from, &to);

  let mut file = File::create("README.md").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

pub fn bump_version (version: &str) {
  bump_build_gradle_kts(version);
  bump_readme(version);
}
