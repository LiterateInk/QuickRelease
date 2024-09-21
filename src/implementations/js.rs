use std::fs;

/// Reads the `package.json` file and parses it as JSON
/// and returns the value of the `version` property as string.
pub fn get_current_version () -> String {
  // Read directly for current directory.
  let file = fs::File::open("package.json")
    .expect("file should open read only");

  let json: serde_json::Value = serde_json::from_reader(file)
      .expect("file should be proper JSON");
  
  let version = json.get("version")
      .expect("'package.json' is missing 'version' property.");

  version.as_str().unwrap().to_string()
}