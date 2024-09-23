use std::{io::Error, process::{Command, Output}};

// takes an array of &str and pass them as arg
pub fn git (args: &[&str]) -> Result<Output, Error> {
  println!("+> git {}", args.join(" "));

  Command::new("git")
    .args(args)
    .output()
}