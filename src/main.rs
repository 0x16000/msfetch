use owo_colors::OwoColorize;
use std::process::Command;
#[allow(dead_code)]
#[cfg(target_os = "macos")]

mod colors {
    pub const CYAN: &str = "\x1b[36m";
}

fn main() {
    fetch();
}
fn fetch() {
    let asciiart = r#"    /\
   /  \ 
  /    \
  \    /
   \  /
    \/"#;
    let ascii = asciiart;

    println!("{}", colors::CYAN);
    let namey = "User".cyan();
    let memy = "Memory".green();
    let kernely = "Kernel".yellow();
    let shelly = "Shell".purple();

    let output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");
    let username = String::from_utf8_lossy(&output.stdout);

    let name = Command::new("uname")
        .arg("-n")
        .output()
        .expect("Failed to execute command");
    let hostname = String::from_utf8_lossy(&name.stdout);

    let mem = Command::new("sysctl")
        .arg("hw.memsize")
        .output()
        .expect("Failed to execute command");
    let mem_string = String::from_utf8(mem.stdout).unwrap();
    let memory = mem_string.trim().split(": ").collect::<Vec<&str>>()[1]
        .parse::<u64>()
        .unwrap()
        / 1024
        / 1024;

    let oskernel = Command::new("uname")
        .output()
        .expect("Failed to execute command");
    let os = String::from_utf8_lossy(&oskernel.stdout);

    let oskernelname = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    let os_name = String::from_utf8_lossy(&oskernelname.stdout);

    let shell_output = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute command");
    let shell = String::from_utf8_lossy(&shell_output.stdout);

    let side_text = [
        format!("{}: {}@{}", namey, username.trim(), hostname.trim()),
        format!("{}: {} MB", memy, memory),
        format!("{}: {} {}", kernely, os.trim(), os_name.trim()),
        format!("{}: {}", shelly, shell.trim()),
    ];
    let ascii_lines: Vec<&str> = ascii.lines().collect();
    let total_lines = ascii_lines.len();
    let text_lines = side_text.len();
    let start = (total_lines - text_lines) / 2;

    for (i, line) in ascii_lines.iter().enumerate() {
        if i >= start && i < start + text_lines {
            let text = &side_text[i - start];
            println!("{:<8}  {:<3}", line, text);
        } else {
            println!("{}", line);
        }
    }
}
