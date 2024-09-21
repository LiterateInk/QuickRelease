use std::fmt;
use git2::Repository;

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

pub fn detect_language (repo: &Repository) -> Language {
  let head = repo.head()
    .expect("Failed to get HEAD reference");

  let branch = head.shorthand()
    .expect("Failed to get branch name from HEAD reference");

  match branch {
    "js" => Language::JS,
    "rust" => Language::Rust,
    "kotlin" => Language::Kotlin,
    _ => panic!("Unknown branch, make sure to checkout to a valid branch."),
  }
}
