use array_tool::vec::Intersect;
use clap::Parser;
use git_utils_shared as git_utils;
use pathdiff::diff_paths;
use std::path::PathBuf;
use std::process::Command;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// args to pass to git
    args: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let cwd = PathBuf::from(&std::env::current_dir().unwrap());

    let mut embedded_repo = None;
    let mut egit_path = None;

    for parent_folder in cwd.ancestors() {
        if parent_folder.join(".egit").is_dir() {
            embedded_repo = Some(parent_folder);
            egit_path = Some(parent_folder.join(".egit"));
            break;
        }
    }

    match embedded_repo.and(egit_path.clone()) {
        Some(_) => {}
        None => {
            eprintln!("{} is not an embedded directory", cwd.display());
            std::process::exit(1);
        }
    }

    let cmd = Command::new("git")
        .args(&args.args)
        .env("GIT_DIR", &egit_path.unwrap())
        .env("GIT_WORK_TREE", &embedded_repo.unwrap())
        .status()
        .expect("Failed git command inside egit");

    if cmd.success() {
        let head =
            git_utils::embed::get_head_of_embed_project(&embedded_repo.unwrap().to_string_lossy());

        let to_change_head_commands = vec![
            "reset".to_string(),
            "checkout".to_string(),
            "commit".to_string(),
            "merge".to_string(),
            "rebase".to_string(),
            "ff".to_string(),
            "cherry-pick".to_string(),
        ];

        // check if any of head changing commands are in the passed args
        if !args.args.intersect(to_change_head_commands).is_empty() {
            print!("Head has_changed");
            let parent_repo = git_utils::repo_top_level_dir().unwrap();
            // todo: modifie and use _> git_utils::embed::add_fild_to_embed_file();
            let _ = Command::new("git")
                .args([
                    "config",
                    "--file",
                    &parent_repo
                        .join(".gitembed")
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                    &format!(
                        "embed.{}.head",
                        diff_paths(embedded_repo.unwrap(), parent_repo)
                            .unwrap()
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                    ),
                    &head,
                ])
                .status()
                .expect("Failed")
                .success();
        }
    }

    std::process::exit(cmd.code().unwrap())
}
