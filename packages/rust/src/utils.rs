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

pub fn unix_find_pid_on_port(port: u32) -> Result<Option<String>, String> {
    let command = Command::new("lsof")
        .arg(format!("-i:{}", port))
        .arg("-t")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Failed to execute command: {}", err))?
        .wait_with_output()
        .map_err(|err| format!("Failed to wait for command: {}", err))?;

    let mut pid =
        String::from_utf8(command.stdout).map_err(|err| format!("Failed to get PID: {}", err))?;

    // Trim the trailing \n of the PID string
    trim_newline(&mut pid);

    if pid.is_empty() {
        Ok(None)
    } else {
        Ok(Some(pid))
    }
}

pub fn unix_kill_process_with_pid(pid: &str, force: bool) -> Result<(), String> {
    let mut command = Command::new("kill");

    if force {
        command.arg("-9");
    }

    let output = command
        .arg(pid)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Failed to execute command: {}", err))?
        .wait_with_output()
        .map_err(|err| format!("Failed to wait for command: {}", err))?;

    let result = String::from_utf8(output.stdout)
        .map_err(|err| format!("Failed to get command output: {}", err))?;

    if result.is_empty() {
        Ok(())
    } else {
        Err(format!("Failed to kill process with PID {}", pid))
    }
}
