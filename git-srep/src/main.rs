use clap::Parser;
use git_utils_shared as git_utils;
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
        help = "Replace words even if the matched file is modified (file not staged)"
    )]
    force: bool,

    #[clap(default_value = ".")]
    paths: Vec<String>,
}

fn change_word_in_files(file_path: Vec<String>, search: &str, replace: &str) {
    let _ = Command::new("sed")
        .args([
            "-i",
            &format!(
                "s/{}/{}/g",
                search.replace("/", r"\/"),
                replace.replace("/", r"\/")
            ),
        ])
        .args(file_path)
        .status()
        .expect("Failed ")
        .success();
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let files = git_utils::get_files_with_word(&args.search, &args.paths);

    println!("{files:?}");

    match files {
        Some(files) => {
            if git_utils::file::files_are_modified(&files) && !args.force {
                println!(
                    "There are not staged changes, in the matched files: {:?} ",
                    files
                );
            } else {
                change_word_in_files(files, &args.search, &args.replace);
            }
        }
        None => {
            println!("There is not any file containing the given word.");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::Path, process::Command};

    fn git_init() {
        let git_init = Command::new("git")
            .arg("init")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(git_init.success());
    }
    fn git_add() {
        let git_init = Command::new("git").args(["add", "-A"]).status().unwrap();
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
        let mut args = vec![
            "run",
            "--release",
            "--bin",
            "git-srep",
            "--",
            search,
            replace,
            path,
        ];
        if force {
            args.push("-f");
        }

        Command::new("cargo")
            .args(args)
            // .stdout(std::process::Stdio::null())
            // .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
    }

    fn create_dir_structure() {
        if let Err(_err) = fs::remove_dir_all(".cargo_test") {}
        create_folder_and_move_into_it(".cargo_test");
        create_folder("subfolder");
        write_file("subfolder/b.txt", "test_a");
        git_init();
        git_add();
        write_file("a.txt", "test_a");
    }

    fn delete_dir_structure() {
        assert!(env::set_current_dir(&Path::new("..")).is_ok());
        fs::remove_dir_all(".cargo_test").unwrap();
    }

    #[test]
    fn with_git() {
        create_dir_structure();

        run_program("test_a", "new_test_a", "subfolder", false);
        assert_eq!(read_file("subfolder/b.txt"), "new_test_a".to_string());
        assert_eq!(read_file("a.txt"), "test_a".to_string());
        git_add();
        run_program("test_a", "new_test_a", ".", false);
        assert_eq!(read_file("a.txt"), "new_test_a".to_string());

        delete_dir_structure();
    }
}
