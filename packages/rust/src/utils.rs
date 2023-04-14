use std::process::Command;
use std::process::Stdio;

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn unix_find_pid_on_port(port: u32) -> Option<String> {
    let pid_command = Command::new("lsof")
        .arg(format!("-i:{}", port))
        .arg("-t")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command")
        .wait_with_output()
        .expect("Failed to wait for command");

    let mut pid = String::from_utf8(pid_command.stdout).expect("Failed to get PID");

    // Trim the trailing \n of the PID string
    trim_newline(&mut pid);

    if pid.is_empty() {
        None
    } else {
        Some(pid)
    }
}
