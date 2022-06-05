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
pub fn modified_files(file_paths: &Vec<String>) -> Option<Vec<String>> {
    let stdout_raw = Command::new("git")
        .args(["--no-pager", "diff", "--name-only", "-z"])
        .args(file_paths)
        .output()
        .expect("Failed ")
        .stdout;
    let stdout_str = String::from_utf8(stdout_raw).unwrap();

    if stdout_str.is_empty() {
        return None;
    }
    let files = stdout_str[..stdout_str.len() - 1]
        .split("\0")
        .map(|path| path.to_owned())
        .collect::<Vec<String>>();
    Some(files)
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
