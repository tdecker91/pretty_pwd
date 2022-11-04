use crate::fancier::fancy_directory;
use crate::fancier::get_fancy_working_directory;
use crate::environment::Env;
use crate::config::{PpwdConfig};
use std::path::PathBuf;

struct MockEnv {
  dir: String,
  home: String
}

impl Env for MockEnv {
  fn cwd(&self) -> String {
    self.dir.to_string()
  }

  fn home_dir(&self) -> String {
    self.home.to_string()
  }
  
  fn home_path(&self) -> Option<PathBuf> {
    None
  }
}

#[test]
fn it_colorizes_text() {
  let config = PpwdConfig::default();

  println!("{}", fancy_directory("~/folder".to_string(), &config));

  assert_eq!("\u{1b}[34m~\u{1b}[0m\u{1b}[31m/\u{1b}[0m\u{1b}[36mfolder\u{1b}[0m", fancy_directory("~/folder".to_string(), &config));
}

#[test]
fn it_replaces_home_directory() {
  let mock_env = MockEnv { 
    dir: String::from("/Users/beavis/Documents/Users/beavis"),
    home: String::from("/Users/beavis")
  };

  let config = PpwdConfig {
    truncate_home: Some(true),
    tilde: None,
    tilde_bg: None,
    path_separator: None,
    path_separator_bg: None,
    dir_name: None,
    dir_name_bg: None
  };

  println!("{}", get_fancy_working_directory(&mock_env, &config));

  assert_eq!(
    String::from("~/Documents/Users/beavis"),
    get_fancy_working_directory(&mock_env, &config)
  );
}

#[test]
fn it_ignores_home_directory() {
  let mock_env = MockEnv { 
    dir: String::from("/Users/beavis/Documents/Users/beavis"),
    home: String::from("/Users/beavis")
  };

  let config = PpwdConfig {
    truncate_home: Some(false),
    tilde: None,
    tilde_bg: None,
    path_separator: None,
    path_separator_bg: None,
    dir_name: None,
    dir_name_bg: None
  };

  println!("{}", get_fancy_working_directory(&mock_env, &config));

  assert_eq!(
    String::from("/Users/beavis/Documents/Users/beavis"),
    get_fancy_working_directory(&mock_env, &config)
  );
}