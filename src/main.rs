use git2::Repository;

mod language;
use language::{detect_language, Language};

mod implementations;
use implementations::js;
use implementations::kotlin;

fn main() {
  let repo = Repository::open_from_env()
    .expect("Failed to open repository, make sure you're in a project where a repository has been initialized.");

  let language = detect_language(&repo);
  
  println!("=> Automatically detected {language}");

  let version = match language {
    Language::JS => js::get_current_version(),
    Language::Kotlin => kotlin::get_current_version(),
    _ => panic!("Unsupported language, make sure to checkout to a valid branch."),
  };

  println!("=> Current version is {version}");
}
