use std::io::{Error, ErrorKind};
use std::process::Command;
use std::process::Stdio;

#[cfg(target_os = "linux")]
pub fn trim_newline(s: &mut String) {
    if let Some('\n') | Some('\r') = s.chars().rev().next() {
        s.pop();
        if let Some('\r') = s.chars().rev().next() {
            s.pop();
        }
    }
}

#[cfg(target_os = "linux")]
pub fn unix_find_pid_on_port(port: u32) -> Result<Option<String>, Error> {
    let command = Command::new("lsof")
        .arg(format!("-i:{}", port))
        .arg("-t")
        .stdout(Stdio::piped())
        .spawn()?;

    let output = command.wait_with_output()?;

    if output.status.success() {
        let mut pid = String::from_utf8(output.stdout)
            .map_err(|err| Error::new(ErrorKind::Other, format!("Failed to get PID: {}", err)))?;

        // Trim the trailing \n of the PID string
        trim_newline(&mut pid);

        if pid.is_empty() {
            Ok(None)
        } else {
            Ok(Some(pid))
        }
    } else {
        Err(Error::new(ErrorKind::Other, "Failed to execute command"))
    }
}

#[cfg(target_os = "linux")]
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

/// Returns info about the process running on the given port on Windows.
#[cfg(target_os = "windows")]
pub fn windows_find_pid_on_port(port: u32) -> Result<Option<String>, Error> {
    let command = Command::new("netstat")
        .arg("-ano")
        .stdout(Stdio::piped())
        .spawn()?;
    let output = command.wait_with_output()?;
    let netstat_output = String::from_utf8(output.stdout).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Failed to convert output to string: {}", e),
        )
    })?;

    let pid = netstat_output
        .lines()
        .filter(|line| line.contains(&format!(":{}", port)))
        .find_map(|line| {
            let columns: Vec<&str> = line.split_whitespace().collect();
            match columns.len() {
                5 => Some(columns[4]),
                _ => None,
            }
        });

    match pid {
        Some(pid) => Ok(Some(pid.to_string())),
        None => Ok(None),
    }
}

#[cfg(target_os = "windows")]
pub fn windows_kill_process_with_pid(pid: &str) -> Result<(), String> {
    let command = Command::new("taskkill")
        .args(&["/F", "/PID", pid])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Failed to execute command: {}", err))?
        .wait_with_output()
        .map_err(|err| format!("Failed to wait for command: {}", err))?;

    let taskkill_output = String::from_utf8(command.stdout)
        .map_err(|err| format!("Failed to get output: {}", err))?;

    if taskkill_output.contains("SUCCESS") {
        Ok(())
    } else {
        Err(format!("Failed to kill process with PID {}", pid))
    }
}
