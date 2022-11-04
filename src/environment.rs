use std::env;
use std::path::PathBuf;

pub trait Env {
  fn cwd(&self) -> String;
  fn home_dir(&self) -> String;
  fn home_path(&self) -> Option<PathBuf>;
  fn config_path(&self) -> String;
}

pub struct ShellEnv {}

impl Env for ShellEnv {
  fn cwd(&self) -> String {
    return match env::var("PWD") {
      Ok(v) => v,
      Err(e) => panic!("${} is not set ({})", "PWD", e)
    }
  }

  fn home_dir(&self) -> String {
    return match env::var("HOME") {
      Ok(v) => v,
      Err(_e) => "".to_owned()
    }
  }

  fn home_path(&self) -> Option<PathBuf> {
    return home::home_dir()
  }

  fn config_path(&self) -> String {
    return match env::var("PPWD_CONFIG") {
      Ok(v) => v,
      Err(_e) => "~/.ppwd".to_owned()
    }
  }
}