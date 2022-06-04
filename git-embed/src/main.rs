use clap::Parser;
use colored::Colorize;
use git_utils_shared as git_utils;
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Add(Add),
    Init,
    Fetch,
    Status,
}

#[derive(clap::Args, Debug)]
struct Add {
    /// Git url of repository to embed.
    git_url: String,
    /// Path to where code should be placed.
    #[clap(validator(|path| match std::path::Path::new(&path).is_dir() {
        true => Err("Folder already exists"),
        false => Ok(())
    }))]
    project_path: String,
}

fn main() {
    let args = Args::parse();
    use Action::*;
    match args.action {
        Add(add_args) => add(&add_args),
        Init => init(),
        Fetch => fetch(),
        Status => status(),
    }
}

fn add(add_args: &Add) {
    git_utils::embed::add_fild_to_embed_file(&add_args.project_path, "url", &add_args.git_url);
    git_utils::embed::add_fild_to_embed_file(
        &add_args.project_path,
        "path",
        &add_args.project_path,
    );

    git_utils::clone(&add_args.git_url, &add_args.project_path);
    std::fs::rename(
        &format!("{}/.git", &add_args.project_path),
        &format!("{}/.egit", &add_args.project_path),
    )
    .unwrap();

    let embed_head = git_utils::embed::get_head_of_embed_project(&add_args.project_path);
    git_utils::embed::add_fild_to_embed_file(&add_args.project_path, "head", &embed_head);
}

fn init() {}

fn fetch() {}

fn status() {}
