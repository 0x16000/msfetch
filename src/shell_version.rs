use std::process::Command;

pub fn get_shell_version() -> String {


    let output = Command::new("sh")
        .arg("-c")
        .arg("zsh --version")
        .output()
        .expect("Failed to execute command");
    let shell_version =   String::from_utf8_lossy(&output.stdout).to_string();
    shell_version
    .trim()
    .split(" ")
    .nth(1)
    .unwrap()
    .to_string()
    
}
