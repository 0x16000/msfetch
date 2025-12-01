use std::process::Command;
#[cfg(target_os = "macos")]
pub fn get_cpu() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("system_profiler SPHardwareDataType | grep -i Apple")
        .output()
        .expect("Failed to execute command");
   let output_str = String::from_utf8_lossy(&output.stdout).to_string();
   output_str
   .trim()
   .split(": ")
   .nth(1)
   .unwrap()
   .trim()
   .to_string()
}
#[cfg(target_os = "linux")]
pub fn get_cpu() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("uname --processor")
        .output()
        .expect("Failed to execute command");
   let output_str = String::from_utf8_lossy(&output.stdout).to_string();
}
#[cfg(target_os = "freebsd")]
pub fn get_cpu() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sysctl hw.model")
        .output()
        .expect("Failed to execute command");
   let output_str = String::from_utf8_lossy(&output.stdout).to_string();
   output_str
   .trim()
   .split(": ")
   .nth(1)
   .unwrap()
   .trim()
   .to_string()
}
#[cfg(target_os = "openbsd")]
pub fn get_cpu() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sysctl hw.model")
        .output()
        .expect("Failed to execute command");
   let output_str = String::from_utf8_lossy(&output.stdout).to_string();
   output_str
   .trim()
   .split(": ")
   .nth(1)
   .unwrap()
   .trim()
   .to_string()
}