use clap::Parser;
use colored::Colorize;
use git_utils_shared as git_utils;
use rayon::prelude::*;
use std::process::Command;

/// Replace given phrase inside files of the provided paths.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The word to substitute
    search: String,
    /// The word to substitute with
    replace: String,

    #[clap(
        long,
        short,
        takes_value = false,
        help = "Substituted word in git dirty files"
    )]
    force: bool,

    #[clap(required = true)]
    paths: Vec<String>,
}

fn is_file_dirty(file_path: &str) -> bool {
    let modified = git_utils::file::file_is_modified(file_path);
    let untracked = git_utils::file::file_is_untracked(file_path);
    if modified {
        println!("modified -> {}", file_path.red());
    }
    if untracked {
        println!("Untracked -> {}", file_path.red())
    }

    modified || untracked
}

fn change_word_in_files(file_path: Vec<String>, search: &str, replace: &str) {
    // println!(
    //     "Replacing {} -> {} in {}",
    //     search,
    //     replace,
    //     file_path.green()
    // );

    let change_success = Command::new("sed")
        .args(["-i", &format!("s/{search}/{replace}/g")])
        .args(file_path)
        .status()
        .expect("Failed ")
        .success();

    // if !change_success {
    //     panic!(
    //         "Error in replacing {} -> {} in {}",
    //         search,
    //         replace,
    //         file_path.red()
    //     )
    // }
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    // let files = git_utils::get_files_with_word_in_it(&args.search);

    let files: Vec<String> = args
        .paths
        .par_iter()
        .map(|path| {
            if args.force {
                git_utils::get_files_with_word_using_grep(&args.search, path)
            } else {
                git_utils::get_files_with_word(&args.search, path)
            }
        })
        .filter(|file| file.is_some())
        .map(|files| files.unwrap())
        .flatten()
        .collect();

    println!("{files:?}");
    if files.is_empty() {
        println!("There is not any file containing the given word or all files are not staged. Try running 'git add .' ");
        std::process::exit(1);
    }

    let dirty_files = files.par_iter().any(|file| is_file_dirty(file));
    if dirty_files && !args.force {
        println!(
            "There is a dirty file (untracked or modified), can't perform replacement. Use --force to ovveride this feature"
        )
    } else {
        change_word_in_files(files, &args.search, &args.replace);
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::Path, process::Command};

    fn git_init(folder: &str) {
        let git_init = Command::new("git")
            .args(["init", folder])
            // .stdout(std::process::Stdio::null())
            // .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(git_init.success());
    }
    fn git_add() {
        let git_init = Command::new("git").args(["add", "."]).status().unwrap();
        assert!(git_init.success());
    }

    fn create_folder(name: &str) {
        fs::create_dir_all(name).unwrap();
    }
    fn create_folder_and_move_into_it(name: &str) {
        fs::create_dir_all(name).unwrap();
        assert!(env::set_current_dir(&Path::new(name)).is_ok());
    }
    fn write_file(file: &str, content: &str) {
        fs::write(file, content).expect("Unable to write file");
    }

    fn read_file(file: &str) -> String {
        fs::read_to_string(file).expect("Unable to read file")
    }

    fn run_program(search: &str, replace: &str, path: &str, force: bool) {
        Command::new("cargo")
            .args([
                "run",
                "--release",
                "--bin",
                "git-srep",
                "--",
                search,
                replace,
                path,
                if force { "-f" } else { "" },
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
    }

    fn create_dir_structure() {
        if let Err(_err) = fs::remove_dir_all("temp_git_test") {}
        create_folder("temp_git_test");
        create_folder("temp_git_test/subfolder");
        write_file("temp_git_test/subfolder/b.txt", "test_a");
        git_init("temp_git_test");
        git_add();
        write_file("temp_git_test/a.txt", "test_a");
    }

    fn delete_dir_structure() {
        fs::remove_dir_all("temp_git_test").unwrap();
    }

    #[test]
    fn with_git() {
        create_dir_structure();

        run_program("test_a", "new_test_a", "temp_git_test/subfolder", false);
        assert_eq!(
            read_file("temp_git_test/subfolder/b.txt"),
            "new_test_a".to_string()
        );
        assert_eq!(read_file("temp_git_test/a.txt"), "test_a".to_string());
        git_add();
        run_program("test_a", "new_test_a", "temp_git_test", false);
        assert_eq!(read_file("temp_git_test/a.txt"), "new_test_a".to_string());

        delete_dir_structure();
    }
}
