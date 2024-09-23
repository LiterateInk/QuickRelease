use std::fs;

fn read_package_json () -> serde_json::Value {
  // Read directly for current directory.
  let file = fs::File::open("package.json")
    .expect("file should open read only");

  serde_json::from_reader(file)
    .expect("file should be proper JSON")
}

/// Reads the `package.json` file and parses it as JSON
/// and returns the value of the `version` property as string.
pub fn get_current_version () -> String {
  let json = read_package_json();
  
  let version = json.get("version")
      .expect("'package.json' is missing 'version' property.");

  version.as_str().unwrap().to_string()
}

/// Edits the `package.json` file and updates the value of the `version` property.
pub fn bump_version (version: &str) {
  let mut json = read_package_json();

  json["version"] = serde_json::json!(version);

  let file = fs::File::create("package.json")
    .expect("file should open write only");

  serde_json::to_writer_pretty(file, &json)
    .expect("file should be written properly");  
}