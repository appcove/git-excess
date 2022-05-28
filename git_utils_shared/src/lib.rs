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

pub fn get_files_with_word(search: &str, path: &str) -> Option<Vec<String>> {
    let cmd = Command::new("git")
        .args([
            "--no-pager",
            "grep",
            "--full-name",
            "--name-only",
            search,
            path,
        ])
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

pub fn get_files_with_word_in_it_not_git(search: &str, path: &str) -> Option<Vec<String>> {
    let cmd = Command::new("grep")
        .args([search, path, "-lR"])
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
            .map(|path| path.to_owned().replace("./", ""))
            .collect::<Vec<String>>(),
    )
}

pub mod file {
    use std::process::Command;
    pub fn file_is_modified(file_path: &str) -> bool {
        !Command::new("git")
            .args(["diff", "--quiet", &file_path])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .expect("Failed ")
            .success()
    }

    pub fn file_is_tracked(file_path: &str) -> bool {
        Command::new("git")
            .args(["ls-files", "--error-unmatch", &file_path])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .expect("Failed ")
            .success()
    }

    pub fn file_is_untracked(file_path: &str) -> bool {
        !file_is_tracked(file_path)
    }
}
