use std::process::Command;
pub fn run_egit(args: Vec<String>) {
    let _ = Command::new("egit").arg("--").args(args).spawn();
}
