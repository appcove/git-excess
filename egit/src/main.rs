use clap::Parser;
use std::process::Command;
use std::{env, path::Path};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// args to pass to git
    args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let egit_path = Path::new(&std::env::current_dir().unwrap()).join(".egit");
    if !egit_path.is_dir() {
        eprintln!(
            "{} is not an embedded directory",
            egit_path.parent().unwrap().display()
        );
        std::process::exit(1);
    };

    env::set_var("GIT_DIR", &egit_path);
    env::set_var("GIT_WORK_TREE", &egit_path.parent().unwrap());

    let exit_code_git = Command::new("git")
        .args(args.args)
        .status()
        .expect("Failed git command inside egit")
        .code()
        .unwrap();

    std::process::exit(exit_code_git)
}
