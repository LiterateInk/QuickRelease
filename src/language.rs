use std::fmt;
use crate::git;

pub enum Language {
  Rust,
  Kotlin,
  JS,
}

impl fmt::Display for Language {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      Self::Rust => "Rust",
      Self::Kotlin => "Kotlin",
      Self::JS => "JS/TS",     
    })
  }
}

impl Language {
  pub fn from_branch_name (name: &str) -> Self {
    match name {
      "js" => Language::JS,
      "rust" => Language::Rust,
      "kotlin" => Language::Kotlin,
      _ => panic!("Unknown branch, make sure to checkout to a valid branch."),
    }
  }

  pub fn to_branch_name (&self) -> &str {
    match self {
      Language::JS => "js",
      Language::Rust => "rust",
      Language::Kotlin => "kotlin",
    }
  }
}

pub fn detect_language () -> Language {
  let output = git(&["rev-parse", "--abbrev-ref", "HEAD"])
    .expect("Failed to get branch name.");

  let branch_name = String::from_utf8_lossy(&output.stdout);
  let branch_name = branch_name.trim();

  Language::from_branch_name(branch_name)
}
