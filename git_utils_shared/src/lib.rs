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

pub fn clone(git_url: &str, path: &str) {
    let _ = Command::new("git")
        .args(["clone", "--quiet", git_url, path])
        .status()
        .expect("Failed")
        .success();
}

pub mod embed {
    use std::process::Command;

    use itertools::Itertools;
    pub fn add_fild_to_embed_file(project: &str, fild: &str, content: &str) {
        let _ = Command::new("git")
            .args([
                "config",
                "--file",
                ".gitembed",
                &format!("embed.{}.{}", project, fild),
                content,
            ])
            .status()
            .expect("Failed")
            .success();
    }

    pub fn remove_fild_to_embed_file(project: &str, fild: &str) {
        let _ = Command::new("git")
            .args([
                "config",
                "--file",
                ".gitembed",
                "--unset",
                &format!("embed.{}.{}", project, fild),
            ])
            .status()
            .expect("Failed")
            .success();
    }

    pub fn remove_section_to_embed_file(project: &str) {
        let _ = Command::new("git")
            .args([
                "config",
                "--file",
                ".gitembed",
                "--remove-section",
                &format!("embed.{project}"),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .expect("Failed")
            .success();
    }

    pub fn get_head_of_embed_project(embed_project_path: &str) -> String {
        let stdout_raw = Command::new("git")
            .args([
                "--git-dir",
                &format!("{}/.egit", embed_project_path),
                "rev-parse",
                "HEAD",
            ])
            .output()
            .expect("Failed")
            .stdout;
        String::from_utf8(stdout_raw).unwrap().trim().to_string()
    }

    #[derive(Debug)]
    pub struct EmbedEnty {
        pub name: String,
        pub git_url: String,
        pub path: String,
        pub head: String,
    }
    pub fn get_embeds() -> Vec<EmbedEnty> {
        let stdout_raw = Command::new("git")
            .args(["config", "--file", ".gitembed", "--list"])
            .output()
            .expect("Failed")
            .stdout;
        let stdout = String::from_utf8(stdout_raw).unwrap().trim().to_string();

        let mut embeds = Vec::new();

        for (url_line, path_line, head_line) in stdout.lines().tuples() {
            let get_value = |line: &str| -> String {
                line.split("=")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .to_string()
            };

            let git_url = get_value(url_line);
            let path = get_value(path_line);
            let head = get_value(head_line);

            let name = path_line
                .split("=")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .to_string();

            embeds.push(EmbedEnty {
                name,
                git_url,
                path,
                head,
            });
        }
        embeds
    }
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
}
