use std::{
    io::Read,
    process::{Child, Command, Stdio},
};

fn main() {
    let mut command = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run: cargo install --list");
    command.wait().expect("command failed");
    let mut out = "".to_string();
    _ = command
        .stdout
        .expect("failed to get stdout")
        .read_to_string(&mut out)
        .expect("Invalid output from cargo");

    let mut install = Command::new("cargo")
        .arg("install")
        .args(
            out.split_terminator('\n')
                .filter(|s| !s.starts_with(" "))
                .map(|s| s.split(' ').next().unwrap()),
        )
        .spawn()
        .expect("Failed to start install");
    install.wait().expect("Failed to install updates");
}
