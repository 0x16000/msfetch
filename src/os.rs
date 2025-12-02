use std::fs;
use std::process::Command;
#[cfg(target_os = "macos")]
pub fn get_os() -> String {
    let output = Command::new("sw_vers")
        .arg("-productName")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
    
}
#[cfg(target_os = "linux")]
pub fn get_os() {

let os = fs::read_to_string("/etc/os-release").unwrap_or_default() ;
let distro = os
 .lines()
 .find(|line| line.starts_with("ID="))
 .map(|line| line.trim_start_matches("ID=").trim_matches('"'))
 .unwrap_or("Unknown");

}
#[cfg(target_os = "freebsd")]
pub fn get_os() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
    
}
#[cfg(target_os = "openbsd")]
pub fn get_os() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
    
}