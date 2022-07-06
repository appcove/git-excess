use std::{borrow::Cow, thread, time::Duration};

use clap::Parser;
use colored::Colorize;
use git_utils_shared as git_utils;
use tabled::{Style, Table};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Init,
    /// Add an embedded repo.
    Add(Add),
    Remove(Remove),
    // Fetch,
    Status,
    Update,
    Tag,
    /// Get a list of all embedded repo.
    List,
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

#[derive(clap::Args, Debug)]
struct Remove {
    #[clap(validator(|path| match std::path::Path::new(&path).is_dir() {
        true => Ok(()),
        false => Err("Project folder does not exist")
    }))]
    project_path: String,
    /// Also remove the files in the embedded repository, not just the embedded repository entry.
    #[clap(long, short, takes_value = false)]
    remove_all_files: bool,
}

fn main() {
    if git_utils::not_installed("egit") {
        eprintln!(
            "{} egit is not installed, git-embed depends on it.",
            "error:".red().bold()
        );
        std::process::exit(1);
    }

    let args = Args::parse();
    use Action::*;
    match args.action {
        Add(add_args) => add(&add_args),
        Remove(remove_args) => remove(&remove_args),
        Init => init(),
        // Fetch => fetch(),
        Tag => tag(),
        Status => status(),
        Update => update(),
        List => list_embed(),
    }
}

fn add(add_args: &Add) {
    if git_utils::clone(&add_args.git_url, &add_args.project_path) {
        // todo: we need to make sure that path does not already exist, and that it is within the GIT_DIR
        git_utils::embed::add_fild_to_embed_file(&add_args.project_path, "url", &add_args.git_url);
        git_utils::embed::add_fild_to_embed_file(
            &add_args.project_path,
            "path",
            &add_args.project_path,
        );

        std::fs::rename(
            &format!("{}/.git", &add_args.project_path),
            &format!("{}/.egit", &add_args.project_path),
        )
        .unwrap();

        let embed_head = git_utils::embed::get_head_of_embed_project(&add_args.project_path);
        git_utils::embed::add_fild_to_embed_file(&add_args.project_path, "head", &embed_head);
    } else {
        println!("Failed to clone from: {}", &add_args.git_url);
    }
}

fn remove(remove_args: &Remove) {
    let egit_path = format!("{}/.egit", &remove_args.project_path);
    // if egit exist: folder is a subdirectory. Then remove embed entry from .gitembed
    // Optional: remove folder itself
    if std::path::Path::new(&egit_path).is_dir() {
        let _ = std::fs::remove_dir_all(&egit_path);
        if remove_args.remove_all_files {
            let _ = std::fs::remove_dir_all(&remove_args.project_path);
        }
        git_utils::embed::remove_section_to_embed_file(&remove_args.project_path);
    };
}

fn init() {
    let tmp_folder = git_utils::repo_top_level_dir().unwrap().join("egit-tmp");
    let parent_repo = git_utils::repo_top_level_dir().unwrap();

    // dbg!(basename(&parent_repo.display().to_string(), '/'));
    for entry in git_utils::embed::get_embeds()
        .iter()
        .filter(|entry| !std::path::Path::new(&format!("{}/.egit", entry.path)).is_dir())
    {
        println!("Setting up: {}", entry.name.cyan());
        let _ = std::fs::remove_dir_all(&tmp_folder);
        let _ = std::fs::create_dir_all(&tmp_folder);
        if git_utils::clone(
            &entry.git_url,
            &tmp_folder.join("repo").as_os_str().to_str().unwrap(),
        ) {
            let _ = std::fs::rename(tmp_folder.join("repo/.git"), tmp_folder.join("repo/.egit"));

            git_utils::egit::run_egit(
                vec!["reset", "--mixed", &entry.head],
                tmp_folder.join("repo"),
            );
            git_utils::egit::run_egit(
                vec![
                    "checkout",
                    "-b",
                    &utils::basename(&parent_repo.display().to_string(), '/'),
                ],
                tmp_folder.join("repo"),
            );

            git_utils::egit::run_egit(vec!["branch", "-D", "master"], tmp_folder.join("repo"));
        }

        thread::sleep(Duration::from_millis(10));
        let _ = std::fs::remove_dir_all(&tmp_folder);
    }
}

fn fetch() {}

fn status() {
    for entry in git_utils::embed::get_embeds()
        .iter()
        .filter(|entry| std::path::Path::new(&format!("{}/.egit", entry.path)).is_dir())
    {
        println!("Status of: {}", entry.name.cyan());
        let embedded_repo_path = git_utils::repo_top_level_dir().unwrap().join(&entry.path);

        git_utils::egit::run_egit(vec!["rev-parse", "HEAD"], &embedded_repo_path);
        git_utils::egit::run_egit(vec!["status"], &embedded_repo_path);
    }
}

fn tag() {
    let is_git_embed_modified =
        git_utils::file::modified_files(&vec![".gitembed".to_string()]).is_some();

    if is_git_embed_modified {
        eprintln!(".gitembed is dirty.  Please commit this first.");
        std::process::exit(1)
    }

    let parent_repo = git_utils::repo_top_level_dir()
        .unwrap()
        .display()
        .to_string();
    let project_name = utils::basename(&parent_repo, '/');
    if let Some(date) = git_utils::get_last_commit_time() {
        for entry in git_utils::embed::get_embeds() {
            let tag_name = format!(
                "{}.{}.{}",
                &project_name,
                date.format("%Y%m%d%H%M%S"),
                git_utils::get_head()
            );
            println!("Tagging {} as {}", &project_name.cyan(), tag_name);
            let entry_path = std::env::current_dir().unwrap().join(&entry.path);
            git_utils::egit::run_egit(vec!["tag", &tag_name], &entry_path);
            git_utils::egit::run_egit(vec!["push", "--tags"], &entry_path);
        }
    }
}

fn update() {
    for entry in git_utils::embed::get_embeds()
        .iter()
        .filter(|entry| std::path::Path::new(&format!("{}/.egit", entry.path)).is_dir())
    {
        println!("Updating {} to {}", entry.name.cyan(), entry.head.yellow());

        let embedded_path = git_utils::repo_top_level_dir().unwrap().join(&entry.path);
        git_utils::egit::run_egit(vec!["fetch", "--tags"], &embedded_path);
        git_utils::egit::run_egit(vec!["reset", "--mixed", &entry.head], &embedded_path);
        git_utils::egit::run_egit(vec!["status"], &embedded_path);
    }
}

fn list_embed() {
    let table = Table::new(&git_utils::embed::get_embeds()).with(Style::modern());
    println!("{table}");
}

mod utils {
    use std::borrow::Cow;

    pub fn basename<'a>(path: &'a str, sep: char) -> Cow<'a, str> {
        let mut pieces = path.rsplit(sep);
        match pieces.next() {
            Some(p) => p.into(),
            None => path.into(),
        }
    }
}
