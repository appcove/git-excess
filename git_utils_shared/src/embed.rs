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
