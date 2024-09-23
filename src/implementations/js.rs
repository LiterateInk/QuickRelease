use std::fs;

/// Concerning JS/TS implementations, we should only run `pnpm run checks` command.
/// It'll run `tsc`, `eslint` and sometimes some tests if they're available.
pub fn run_checks () {
  let result = std::process::Command::new("pnpm")
    .arg("run")
    .arg("checks")
    .output();

  if let Err(error) = result {
    panic!("Failed to run 'pnpm run checks' command.\nMake sure you have pnpm installed globally.\n\n{error}");
  }

  let output = result.unwrap();
  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stdout);
    panic!("Failed to run `pnpm run checks` command.\n\n{error}");
  }
}

fn read_package_json () -> serde_json::Value {
  let file = fs::File::open("package.json")
    .expect("File should open read only.");

  serde_json::from_reader(file)
    .expect("File should be proper JSON.")
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
    .expect("File should open write only.");

  serde_json::to_writer_pretty(file, &json)
    .expect("File should be written properly.");  
}
