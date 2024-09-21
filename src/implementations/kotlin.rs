use std::{fs, io::Read};

/// Reads the `library/build.gradle.kts` file and parses it as KTS
/// and returns the value of the `version` property as string.
pub fn get_current_version () -> String {
  // Read directly for current directory.
  let mut file = fs::File::open("library/build.gradle.kts")
    .expect("file should open read only");

  let mut buffer = String::new();
  file.read_to_string(&mut buffer).expect("file should be proper KTS");

  // Find the `version = "x.y.z"` line.
  let version_line = buffer.lines().find(|line| line.contains("version"))
    .expect("'build.gradle.kts' is missing 'version' property.");

  let version = version_line.split("\"")
    .nth(1)
    .expect("version should be wrapped in double quotes.");

  version.to_string()
}