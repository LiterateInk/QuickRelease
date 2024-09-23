use git2::Repository;
use dialoguer::Select;

mod language;
use language::{detect_language, Language};

mod implementations;
use implementations::js;
use implementations::kotlin;

const UNSUPPORTED_LANGUAGE: &str = "Unsupported language, make sure to checkout to a valid branch.";

fn main() {
  let repo = Repository::open_from_env()
    .expect("Failed to open repository, make sure you're in a project where a repository has been initialized.");

  let language = detect_language(&repo);
  println!("# Automatically detected {language}");

  // Run checks before bumping the version.
  match language {
    Language::JS => js::run_checks(),
    _ => panic!("{UNSUPPORTED_LANGUAGE}"),
  };

  // Would've panicked if the checks failed.
  println!("# Checks passed successfully");

  let version = match language {
    Language::JS => js::get_current_version(),
    Language::Kotlin => kotlin::get_current_version(),
    _ => panic!("{UNSUPPORTED_LANGUAGE}"),
  };

  println!("# Current version is {version}");
  let mut version: Vec<u8> = version.split(".").map(|part| part.parse().unwrap()).collect();

  let selection = Select::new()
    .with_prompt("=> Let's bump ")
    .items(&["major", "minor", "patch"])
    .default(2)
    .interact()
    .unwrap();

  match selection {
    // major
    0 => {
      version[0] += 1;
      version[1..].fill(0);
    },
    // minor
    1 => {
      version[1] += 1;
      version[2..].fill(0);
    },
    // patch
    2 => version[2] += 1,
    _ => panic!("Invalid selection."),
  }

  let version = version.iter().map(|part| part.to_string()).collect::<Vec<String>>().join(".");
  println!("# New version is {version}");

  match language {
    Language::JS => js::bump_version(&version),
    _ => panic!("{}", UNSUPPORTED_LANGUAGE),
  }
}
