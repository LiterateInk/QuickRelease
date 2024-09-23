use dialoguer::Select;

mod git;
use git::git;

mod language;
use language::{detect_language, Language};

mod implementations;
use implementations::js;
use implementations::kotlin;

const UNSUPPORTED_LANGUAGE: &str = "Unsupported language, make sure to checkout to a valid branch.";

fn main() {
  let language = detect_language();
  println!("# Automatically detected {language}");
  println!("# Will run checks...");

  match language {
    Language::JS => js::run_checks(),
    _ => panic!("{UNSUPPORTED_LANGUAGE}"),
  };

  // Would've panicked if the checks failed.
  println!("# Checks are all good !");

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

  match language {
    Language::JS => js::bump_version(&version),
    _ => panic!("{}", UNSUPPORTED_LANGUAGE),
  }

  let commit_message = format!("chore: release v{version}");
  let tag_message = format!("Release v{version}");
  let tag_name = format!("{}-v{version}", language.to_branch_name());

  let commands = vec![
    // NOTE: not very safe to add everything, might be great in the future to
    // have this as a separate function depending on the language.
    vec!["add", "."],

    vec!["commit", "-m", &commit_message],
    vec!["tag", "-a", &tag_name, "-m", &tag_message],
    vec!["push", "origin", language.to_branch_name(), "--tags"],
  ];

  for command in commands {
    let output = git(&command).unwrap();

    if !output.status.success() {
      let error = String::from_utf8_lossy(&output.stdout);
      panic!("Failed to run git command.\n\n{error}");
    }
  }
}
