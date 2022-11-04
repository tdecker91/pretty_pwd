mod environment;
mod fancier;
mod config;

use environment::{ShellEnv};
use crate::environment::{Env};

fn main() {
    let env = ShellEnv{};
    let cfg = config::get_config(env.home_path());
    let output = fancier::get_fancy_working_directory(&env, &cfg);

    println!("{}", output);
}
