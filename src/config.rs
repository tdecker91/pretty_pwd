use serde_derive::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct PpwdConfig {
    pub truncate_home: Option<bool>,
    pub tilde: Option<String>,
    pub tilde_bg: Option<String>,
    pub path_separator: Option<String>,
    pub path_separator_bg: Option<String>,
    pub dir_name: Option<String>,
    pub dir_name_bg: Option<String>
}

impl ::std::default::Default for PpwdConfig {
    fn default() -> Self { Self {
        truncate_home: Some(false),
        tilde: Some(String::from("blue")),
        tilde_bg: None,
        path_separator: Some(String::from("red")),
        path_separator_bg: None,
        dir_name: Some(String::from("cyan")),
        dir_name_bg: None
    } }
}

pub fn get_config(home_dir: Option<PathBuf>) -> PpwdConfig {
  match home_dir {
    Some(mut path) => {
      path.push(".ppwd");
      match confy::load_path(path) {
          Ok(config) => config,
          Err(_e) => {
            return PpwdConfig::default();
          }
      }
    },
    None => PpwdConfig::default()
  }
}