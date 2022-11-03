use std::fmt::Write;
use colored::Colorize;
use colored::ColoredString;
use crate::environment::{Env};

#[cfg(test)]
mod tests;

fn fancy_directory_separator() -> ColoredString {
  return "/".purple()
}

fn fancy_home_dir() -> ColoredString {
  return "~".blue()
}

fn fancy_folder_name(name: String) -> ColoredString {
  return name.cyan()
}

fn substitute_home_dir(home_dir: String, cwd: String) -> String {
  return match home_dir.as_ref() {
      "" => cwd,
      home_dir => cwd.replacen(&home_dir, "~", 1)
  }
}

fn fancy_directory(cwd: String) -> String {
  let split =  cwd.split("/");
  let mut output = String::new();

  for s in split {
      match s.as_ref() {
          "~" => write!(&mut output, "{}", fancy_home_dir()).unwrap(),
          "" => (),
          folder => {
              write!(&mut output, "{}", fancy_directory_separator()).unwrap();
              write!(&mut output, "{}", fancy_folder_name(folder.to_owned())).unwrap();
          }
      }
  }

  return output;
}

pub fn get_fancy_working_directory(environment: &dyn Env) -> String {
  let cwd = environment.cwd();
  let home = environment.home_dir();
  let directory = substitute_home_dir(home, cwd);

  return fancy_directory(directory)
}