use std::process::Command;

// fn run_command(command: &str, args: [&str]) -> () {}

/// Get the merge base between the two provided branches
pub fn get_merge_base(branch1: &str, branch2: &str) -> Option<String> {
    let merge_base_vec = Command::new("git")
        .arg("merge-base")
        .arg(branch1)
        .arg(branch2)
        .output()
        .expect("Failed to run git merge-base")
        .stdout;

    let merge_base = String::from_utf8(merge_base_vec).unwrap();
    let merge_base = merge_base.trim().to_owned();

    if merge_base.trim().is_empty() {
        return None;
    } else {
        return Some(merge_base);
    }
}

/// Show commits on the second branch that are not on the first branch
pub fn show_uncommon_commit_from_other_branch(branch: &str, other_branch: &str) {
    let _cmd1 = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("--pretty=format:%Cgreen%h%Creset %Cred(%an)%Creset [%ad] %Cblue%s%Creset ")
        .arg(branch)
        .arg(format!("^{}", other_branch))
        .status()
        .expect("Failed to retrieve unique commits");
    println!("\n");
}

/// Show the common commit
pub fn show_common_commit(merge_base: &str) {
    let _cmd3 = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("-1")
        .arg(&merge_base)
        .status()
        .expect("Failed to get first common commit");
}

pub fn get_files_with_word(search: &str, paths: &Vec<String>) -> Option<Vec<String>> {
    let cmd = Command::new("git")
        .args([
            "--no-pager",
            "grep",
            "--files-with-matches",
            "--name-only",
            "-E",
            "-e",
            // todo replace ' and " in search
            &search,
            "--",
        ])
        .args(paths)
        .output()
        .expect("Failed")
        .stdout;
    let stdout = String::from_utf8(cmd).unwrap();
    if stdout.is_empty() {
        return None;
    }

    Some(
        stdout
            .trim()
            .split("\n")
            .map(|path| path.to_owned())
            .collect::<Vec<String>>(),
    )
}

pub mod file {
    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    pub struct GitStatusFile {
        name: String,
        status: GitStatus,
    }
    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    pub enum GitStatus {
        Renamed,
        Modified,
        Deleted,
        Added,
        Untracked,
    }
    use std::{collections::HashMap, process::Command};

    pub fn git_status() -> HashMap<GitStatus, GitStatusFile> {
        let cmd = Command::new("git")
            .args(["status", "--porcelain=v2", "-s"])
            .output()
            .expect("Failed ")
            .stdout;
        let stdout = String::from_utf8(cmd).unwrap();
        let mut git_status_record = HashMap::new();
        for line in stdout.split("\n") {
            let line_split: Vec<&str> = line.trim().split(" ").filter(|c| !c.is_empty()).collect();

            dbg!(&line_split);

            use GitStatus::*;
            let status = match line_split[0] {
                "AD" | "D" => Deleted,
                "AM" | "M" => Modified,
                "AR" | "R" => Renamed,
                "A" => Added,
                "??" => Untracked,
                err => panic!("{} is not an implemented git status", err),
            };

            git_status_record.insert(
                status.clone(),
                GitStatusFile {
                    name: line_split[1].to_string(),
                    status: status,
                },
            );
        }
        git_status_record
    }
    pub fn files_are_modified(file_paths: &Vec<String>) -> bool {
        !Command::new("git")
            .args(["diff", "--quiet"])
            .args(file_paths)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .expect("Failed ")
            .success()
    }

    pub fn files_are_tracked(file_paths: &Vec<String>) -> bool {
        Command::new("git")
            .args(["ls-files", "--error-unmatch"])
            .args(file_paths)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .expect("Failed ")
            .success()
    }
}
