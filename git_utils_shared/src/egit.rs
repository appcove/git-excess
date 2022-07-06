use std::{
    ffi::OsStr,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};
pub fn run_egit<I: AsRef<OsStr>, P: AsRef<Path>>(args: Vec<I>, working_dir: P) {
    let mut cmd = Command::new("egit")
        .arg("--")
        .args(args)
        .stdin(Stdio::inherit())
        .current_dir(working_dir)
        .spawn()
        .unwrap();
    cmd.wait().unwrap();
    thread::sleep(Duration::from_millis(10));
}
