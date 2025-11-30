use owo_colors::OwoColorize;
use std::process::Command;

fn main() {
    fetch();
}

fn fetch() {
    // here to change ascii art
    let asciiart = r#"   
    /\
   /  \
  /    \
  \    /
   \  /
    \/"#;

    let username = get_username();
    let hostname = get_hostname();
    let memory = get_memory();
    let (os, os_version) = get_os_info();
    let shell = get_shell();

    let side_text = [
        format!(
            "{}:   {}@{}",
            "User".cyan(),
            username.trim(),
            hostname.trim()
        ),
        format!("{}: {} MB", "Memory".green(), memory),
        format!("{}: {} {}", "Kernel".yellow(), os.trim(), os_version.trim()),
        format!("{}:  {}", "Shell".purple(), shell.trim()),
    ];

    let ascii_lines: Vec<&str> = asciiart.lines().collect();
    let total_lines = ascii_lines.len();
    let text_lines = side_text.len();
    let start = (total_lines - text_lines) / 2;

    for (i, line) in ascii_lines.iter().enumerate() {
        if i >= start && i < start + text_lines {
            let text = &side_text[i - start];
            println!("{:<8}  {}", line.blue(), text);
        } else {
            println!("{}", line.blue());
        }
    }
}

fn get_username() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_hostname() -> String {
    let output = Command::new("uname")
        .arg("-n")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[cfg(target_os = "macos")]
fn get_memory() -> u64 {
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
fn get_memory() -> u64 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("grep MemTotal /proc/meminfo | awk '{print $2}'")
        .output()
        .expect("Failed to execute command");
    let mem_string = String::from_utf8(output.stdout).unwrap();
    mem_string.trim().parse::<u64>().unwrap() / 1024
}
#[cfg(target_os = "freebsd")]
fn get_memory() -> u64 {
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
fn get_memory() -> u64 {
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
fn get_os_info() -> (String, String) {
    let os_output = Command::new("uname")
        .output()
        .expect("Failed to execute command");
    let os = String::from_utf8_lossy(&os_output.stdout).to_string();

    let version_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    let version = String::from_utf8_lossy(&version_output.stdout).to_string();

    (os, version)
}

fn get_shell() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
