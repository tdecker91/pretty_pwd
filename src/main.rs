mod environment;
mod fancier;

use std::path::PathBuf;
use clap::Parser;
use environment::{ShellEnv};

#[derive(Parser, Debug)]
#[command(name = "pwd-rust")]
#[command(author = "Tyson D. <https://github.com/tdecker91>")]
#[command(version = "1.0")]
#[command(about = "Enchances the 'pwd' command", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[clap(long)]
    verbose: bool,
}


fn main() {
    let _args = Args::parse();
    // println!("verbose: {}", args.verbose);

    let env = ShellEnv{};
    let output = fancier::get_fancy_working_directory(&env);

    println!("{}", output);
}
