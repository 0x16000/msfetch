use owo_colors::OwoColorize;
use std::process::Command;
mod mem;
mod cpu;
mod os;
fn main() {
    fetch();
}

fn fetch() {
    // here to change ascii art
    let asciiart = r#"    /\
   /  \
  /    \
  \    /
   \  /
    \/"#;

    let username = get_username();
    let hostname = get_hostname();
    let os_info = os::get_os();
    let memory = mem::get_memory();
    let (os, os_version) = get_os_info();
    let _shell = get_shell();
    let cpu = cpu::get_cpu();

    let side_text = [
        format!(
            "{}:   {}@{}",
            "User".cyan(),
            username.trim(),
            hostname.trim()
        ),
        format!("{}:     {}", "OS".magenta(), os_info.trim()),
        format!("{}: {} {}", "Kernel".yellow(), os.trim(), os_version.trim()),
        format!("{}: {} MB", "Memory".green(), memory),
        format!("{}:    {}", "Cpu".white(), cpu.trim()),
        format!("{}:  {}", "Shell".purple(), _shell.trim()),

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
