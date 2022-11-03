use crate::fancier::fancy_directory;
use crate::fancier::get_fancy_working_directory;
use crate::environment::Env;

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
}

#[test]
fn it_colorizes_text() {
  assert_eq!("\u{1b}[34m~\u{1b}[0m\u{1b}[35m/\u{1b}[0m\u{1b}[36mfolder\u{1b}[0m", fancy_directory("~/folder".to_string()));
}

#[test]
fn it_replaces_home_directory() {
  let mock_env = MockEnv { 
    dir: String::from("/Users/beavis/Documents/Users/beavis"),
    home: String::from("/Users/beavis")
  };

  assert_eq!(
    String::from("\u{1b}[34m~\u{1b}[0m\u{1b}[35m/\u{1b}[0m\u{1b}[36mDocuments\u{1b}[0m\u{1b}[35m/\u{1b}[0m\u{1b}[36mUsers\u{1b}[0m\u{1b}[35m/\u{1b}[0m\u{1b}[36mbeavis\u{1b}[0m"),
    get_fancy_working_directory(&mock_env)
  );
}