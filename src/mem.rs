use std::process::Command;
#[cfg(target_os = "macos")]
 pub fn get_memory() -> u64 {
    let output = Command::new("sysctl")
        .arg("hw.memsize")
        .output()
        .expect("Failed to execute command");
    let mem_string = String::from_utf8(output.stdout).unwrap();
    mem_string
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap()
        / 1024
        / 1024
}

#[cfg(target_os = "linux")]
pub fn get_memory() -> u64 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("grep MemTotal /proc/meminfo | awk '{print $2}'")
        .output()
        .expect("Failed to execute command");
    let mem_string = String::from_utf8(output.stdout).unwrap();
    mem_string.trim().parse::<u64>().unwrap() / 1024
}
#[cfg(target_os = "freebsd")]
pub fn get_memory() -> u64 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sysctl hw.physmem")
        .output()
        .expect("Failed to execute command");
    let mem_string = String::from_utf8(output.stdout).unwrap();
    mem_string
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap()
        / 1024
        / 1024
}
#[cfg(target_os = "openbsd")]
pub fn get_memory() -> u64 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sysctl hw.physmem")
        .output()
        .expect("Failed to execute command");
    let mem_string = String::from_utf8(output.stdout).unwrap();
    mem_string
        .trim()
        .split("=")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap()
        / 1024
        / 1024
}