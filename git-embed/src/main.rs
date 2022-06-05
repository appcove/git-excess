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
    Fetch,
    Status,
    Update,
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
    let args = Args::parse();
    use Action::*;
    match args.action {
        Add(add_args) => add(&add_args),
        Remove(remove_args) => remove(&remove_args),
        Init => init(),
        Fetch => fetch(),
        Status => status(),
        Update => update(),
        List => list_embed(),
    }
}

fn add(add_args: &Add) {
    git_utils::embed::add_fild_to_embed_file(&add_args.project_path, "url", &add_args.git_url);
    git_utils::embed::add_fild_to_embed_file(
        &add_args.project_path,
        "path",
        &add_args.project_path,
    );

    if git_utils::clone(&add_args.git_url, &add_args.project_path) {
        std::fs::rename(
            &format!("{}/.git", &add_args.project_path),
            &format!("{}/.egit", &add_args.project_path),
        )
        .unwrap();

        let embed_head = git_utils::embed::get_head_of_embed_project(&add_args.project_path);
        git_utils::embed::add_fild_to_embed_file(&add_args.project_path, "head", &embed_head);
    }
}

fn remove(remove_args: &Remove) {
    // todo: should be checking if folder/file exist before deleting ?

    let _ = std::fs::remove_dir_all(format!("{}/.egit", &remove_args.project_path));
    // todo: should this error be handled ?
    // {
    //     Ok(_)
    //     | Err(Os {
    //         kind: std::io::ErrorKind::NotFound,
    //         ..
    //     }) => {}
    //     _ => panic!(""),
    // }

    if remove_args.remove_all_files {
        let _ = std::fs::remove_dir_all(&remove_args.project_path);
    }
    git_utils::embed::remove_section_to_embed_file(&remove_args.project_path);
}
fn init() {
    println!("{:?}", git_utils::embed::get_embeds());
    for entry in git_utils::embed::get_embeds()
        .iter()
        .filter(|entry| !std::path::Path::new(&format!("{}/.egit", entry.path)).is_dir())
    {
        println!("Setting up: {}", entry.name.cyan());
        let _ = std::fs::remove_dir_all("egit-tmp");
        let _ = std::fs::create_dir_all("egit-tmp");
        git_utils::clone(&entry.git_url, "egit-tmp/repo");
        let _ = std::fs::rename("egit-tmp/repo/.git", "egit-tmp/repo/.git");
        assert!(std::env::set_current_dir("egit-tmp/repo").is_ok());

        // todo: implement egit
        // subprocess.call(('egit', 'reset', '--mixed', Embed['head']))
        // subprocess.call(('egit', 'checkout', '-b', PROJECT_DIRNAME))
        // subprocess.call(('egit', 'branch', '-D', 'master'))

        std::fs::remove_dir_all(std::env::current_dir().unwrap().parent().unwrap()).unwrap();
    }
}

fn fetch() {}

fn status() {
    let cwd = std::env::current_dir().unwrap();
    for entry in git_utils::embed::get_embeds()
        .iter()
        .filter(|entry| std::path::Path::new(&format!("{}/.egit", entry.path)).is_dir())
    {
        assert!(std::env::set_current_dir(&entry.path).is_ok());
        println!("Status of: {}", entry.name.cyan());
        println!("{:?}", std::env::current_dir().unwrap());

        // todo: implement egit
        // subprocess.call(('egit', 'rev-parse', 'HEAD'))
        // subprocess.call(('egit', 'status'))
        assert!(std::env::set_current_dir(&cwd).is_ok());
    }
}
fn update() {
    let cwd = std::env::current_dir().unwrap();
    for entry in git_utils::embed::get_embeds()
        .iter()
        .filter(|entry| std::path::Path::new(&format!("{}/.egit", entry.path)).is_dir())
    {
        assert!(std::env::set_current_dir(&entry.path).is_ok());
        println!("Updating {} to {}", entry.name.cyan(), entry.head.yellow());
        println!("{:?}", std::env::current_dir().unwrap());

        // todo: implement egit
        // subprocess.call(('egit', 'fetch', '--tags'))
        // subprocess.call(('egit', 'reset', '--mixed', Embed['head']))
        // subprocess.call(('egit', 'status'))
        assert!(std::env::set_current_dir(&cwd).is_ok());
    }
}

fn list_embed() {
    let table = Table::new(&git_utils::embed::get_embeds()).with(Style::modern());
    println!("{table}");
}
