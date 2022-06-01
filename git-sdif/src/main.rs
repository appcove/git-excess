use clap::Parser;
use colored::Colorize;

use git_utils_shared as git_utils;
/// Compare two git branches or commits to see what the symmetric difference is.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The first branch to compare
    branch1: String,
    /// The second branch to compare
    #[clap(default_value = "HEAD")]
    branch2: String,
}

fn cli_divider(message: &str) {
    println!("");
    println!("-------------------------------------------------------------------------------");
    println!("{message}");
    println!("");
}

fn main() {
    let args = Args::parse();

    let merge_base = git_utils::get_merge_base(&args.branch1, &args.branch2);

    match &merge_base {
        Some(commit) => println!("Found {}: {}", "Merge base".cyan(), commit),
        None => {
            eprintln!(
                "!! No merge base found between branches [{} <-> {}]\n",
                args.branch1, args.branch2
            );
            std::process::exit(1);
        }
    }

    cli_divider(&format!(
        "Commits unique to {}",
        args.branch1.to_uppercase().bold().yellow()
    ));

    git_utils::show_uncommon_commit_from_other_branch(&args.branch1, &args.branch2);

    cli_divider(&format!(
        "Commits unique to {}",
        args.branch2.to_uppercase().bold().yellow()
    ));

    git_utils::show_uncommon_commit_from_other_branch(&args.branch2, &args.branch1);

    cli_divider(&format!(
        "Common anchestor of {} and {}",
        &args.branch1, &args.branch2
    ));

    git_utils::show_common_commit(&merge_base.unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::git_utils;
        use std::{env, fs, path::Path, process::Command};

        fn git_init() {
            let git_init = Command::new("git").arg("init").status().unwrap();
            assert!(git_init.success());
        }
        fn commit(message: &str) {
            let git_commit = Command::new("git")
                .args(["commit", "--allow-empty", "-m", message])
                .status()
                .unwrap();
            assert!(git_commit.success());
        }

        fn create_branch_and_move_into_it(branch_name: &str) {
            let new_branch = Command::new("git")
                .args(["checkout", "-b", branch_name])
                .status()
                .unwrap();
            assert!(new_branch.success());
        }

        fn change_branch(branch_name: &str) {
            let branch = Command::new("git")
                .args(["checkout", branch_name])
                .status()
                .unwrap();
            assert!(branch.success());
        }
        fn show_uncommon_commit_from_other_branch(branch: &str, other_branch: &str) -> String {
            let cmd = Command::new("git")
                .arg("--no-pager")
                .arg("log")
                .arg("--pretty=format:%Cgreen%h%Creset %Cred(%an)%Creset [%ad] %Cblue%s%Creset ")
                .arg(branch)
                .arg(format!("^{}", other_branch))
                .output()
                .expect("Failed to retrieve unique commits")
                .stdout;
            String::from_utf8(cmd).unwrap()
        }

        pub fn show_common_commit(merge_base: &str) -> String {
            let cmd = Command::new("git")
                .arg("--no-pager")
                .arg("log")
                .arg("-1")
                .arg(&merge_base)
                .output()
                .expect("Failed to get first common commit")
                .stdout;
            String::from_utf8(cmd).unwrap()
        }

        if let Err(_err) = fs::remove_dir_all(".cargo_test") {};
        // create folder and move into it
        fs::create_dir_all(".cargo_test").unwrap();
        assert!(env::set_current_dir(&Path::new(".cargo_test")).is_ok());

        git_init();
        commit("A");

        // commit in dev
        create_branch_and_move_into_it("dev");
        commit("D");
        commit("E");

        // commit in master
        change_branch("master");
        commit("B");
        commit("C");

        let master = show_uncommon_commit_from_other_branch("master", "dev");
        assert!(master.contains("] C") && master.contains("] B"));
        assert_eq!(master.matches("\n").count(), 1);

        let dev = show_uncommon_commit_from_other_branch("dev", "master");
        assert!(dev.contains("] E") && dev.contains("] D"));
        assert_eq!(master.matches("\n").count(), 1);

        let common_key = git_utils::get_merge_base("dev", "master").unwrap();
        let common_commmit = show_common_commit(&common_key);
        assert!(common_commmit.contains(&common_key));

        println!("{:?}", std::env::current_dir());
        fs::remove_dir_all(std::env::current_dir().unwrap()).unwrap();
    }
}
