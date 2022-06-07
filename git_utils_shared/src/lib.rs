use chrono::{DateTime, FixedOffset};
use std::process::{Command, Stdio};
pub mod egit;
pub mod embed;
pub mod file;
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
    // assert!(search.contains("\""), "search word can't contain '\"' ");
    // assert!(search.contains("'"), "search word can't contain "\'" ");
    let stdout_raw = Command::new("git")
        .args([
            "--no-pager",
            "grep",
            "--files-with-matches",
            "--name-only",
            "-z",
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

    let stdout_str = String::from_utf8(stdout_raw).unwrap();

    if stdout_str.is_empty() {
        return None;
    }

    // Because the output of the command is a string with a null character at the end, remove the last element
    let files = stdout_str[..stdout_str.len() - 1]
        .split("\0")
        .map(|path| path.to_owned())
        .collect::<Vec<String>>();

    Some(files)
}

pub fn clone(git_url: &str, path: &str) -> bool {
    Command::new("git")
        .args(["clone", "--quiet", git_url, path])
        .stdin(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success()
}

pub fn repo_top_level_dir() -> Result<std::path::PathBuf, std::io::Error> {
    let raw_output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?
        .stdout;

    let stdout_str = String::from_utf8(raw_output).unwrap();
    return Ok(std::path::PathBuf::from(stdout_str.trim()));
}

pub fn is_installed(tool: &str) -> bool {
    Command::new(&tool)
        .arg("-V")
        .stdout(std::process::Stdio::null())
        .status()
        .is_ok()
}
pub fn not_installed(tool: &str) -> bool {
    !is_installed(tool)
}

pub fn get_last_commit_time() -> Option<DateTime<FixedOffset>> {
    let stdout_raw = Command::new("git")
        .args(["log", "-1", "--date=iso-strict", "--format=%cd", "HEAD"])
        .output()
        .expect("Failed ")
        .stdout;
    let stdout_str = String::from_utf8(stdout_raw).unwrap();
    if stdout_str.is_empty() {
        return None;
    }
    let date = chrono::DateTime::parse_from_rfc3339(&stdout_str.trim()).unwrap();

    return Some(date);
}

pub fn get_head() -> String {
    let stdout_raw = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("Failed")
        .stdout;
    String::from_utf8(stdout_raw).unwrap().trim().to_string()
}
