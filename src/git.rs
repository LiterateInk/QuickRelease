use std::process::{Command, Output};
use colored::Colorize;

pub fn git (args: &[&str]) -> Output {
  let log = format!("+> git {}", args.join(" "));
  println!("{}", log.bright_black());

  Command::new("git")
    .args(args)
    .output()
    .expect("Failed to run git command, make sure git is installed on your machine.")
}

// get every commit message between the two versions
pub fn diff (branch_name: &str, old_version: &str, new_version: &str) -> String {
  let references = format!("{branch_name}-v{old_version}..{branch_name}-v{new_version}");

  let output = git(&["log", "--oneline", "--pretty=format:%s (%h)", &references]);
  String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn origin_url () -> String {
  let output = git(&["remote", "get-url", "origin"]);
  let url = String::from_utf8_lossy(&output.stdout);

  url.trim().to_string()
}
