use std::fmt::Write;
use colored::Colorize;
use colored::ColoredString;
use crate::environment::{Env};
use crate::config::{PpwdConfig};

#[cfg(test)]
mod tests;

fn colorize_string(value: String, color: &Option<String>) -> ColoredString {
  match color.as_deref() {
    Some("red") => value.red(),
    Some("yellow") => value.yellow(),
    Some("green") => value.green(),
    Some("blue") => value.blue(),
    Some("purple") => value.purple(),
    Some("white") => value.white(),
    Some("black") => value.black(),
    Some("cyan") => value.cyan(),
    _ => value.normal()
  }
}

fn colorize_bg(value: ColoredString, color: &Option<String>) -> ColoredString {
  match color.as_deref() {
    Some("red") => value.on_red(),
    Some("yellow") => value.on_yellow(),
    Some("green") => value.on_green(),
    Some("blue") => value.on_blue(),
    Some("purple") => value.on_purple(),
    Some("white") => value.on_white(),
    Some("black") => value.on_black(),
    Some("cyan") => value.on_cyan(),
    _ => value
  }
}

fn fancy_directory_separator(config: &PpwdConfig) -> ColoredString {
  let string = colorize_string(String::from("/"), &config.path_separator);
  return colorize_bg(string, &config.path_separator_bg);
}

fn fancy_home_dir(config: &PpwdConfig) -> ColoredString {
  let string = colorize_string(String::from("~"), &config.tilde);
  return colorize_bg(string, &config.tilde_bg);
}

fn fancy_folder_name(name: String, config: &PpwdConfig) -> ColoredString {
  let string = colorize_string(name, &config.dir_name);
  return colorize_bg(string, &config.dir_name_bg);
}

fn substitute_home_dir(home_dir: String, cwd: String) -> String {
  return match home_dir.as_ref() {
      "" => cwd,
      home_dir => cwd.replacen(&home_dir, "~", 1)
  }
}

fn fancy_directory(cwd: String, config: &PpwdConfig) -> String {
  let split =  cwd.split("/");
  let mut output = String::new();

  for s in split {
      match s.as_ref() {
          "~" => write!(&mut output, "{}", fancy_home_dir(config)).unwrap(),
          "" => (),
          folder => {
              write!(&mut output, "{}", fancy_directory_separator(config)).unwrap();
              write!(&mut output, "{}", fancy_folder_name(folder.to_owned(), config)).unwrap();
          }
      }
  }

  return output;
}

pub fn get_fancy_working_directory(environment: &dyn Env, config: &PpwdConfig) -> String {
  let mut cwd = environment.cwd();
  let home = environment.home_dir();

  cwd = match config.truncate_home {
    Some(true) => substitute_home_dir(home, cwd),
    _ => cwd
  };

  return fancy_directory(cwd, config)
}