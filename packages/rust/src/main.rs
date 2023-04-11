use clap::Parser;
use std::process::Command;
use std::process::Stdio;

mod utils;
use crate::utils::trim_newline;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Port number to kill process on
    #[arg(short = 'p', long = "port")]
    port: u32,

    /// Kill the process without giving it an opportunity to clean up
    #[arg(short = 'f', long = "force")]
    force: bool,
}

fn main() {
    println!("Running PortPurge");
    let args = Args::parse();

    println!("Port to kill: {}", args.port);

    let pid_command = Command::new("lsof")
        .arg(format!("-i:{}", args.port))
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
        println!("No process running on port {}", args.port);
        std::process::exit(0);
    }

    println!("Found process running with PID: {}", pid);

    let mut kill_command = Command::new("kill");

    if args.force {
        kill_command.arg("-9");
    }

    let kill_command_output = kill_command
        .arg(&pid)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command")
        .wait_with_output()
        .expect("Failed to wait for command");

    let kill_command_result =
        String::from_utf8(kill_command_output.stdout).expect("Failed to get command output");

    if kill_command_result.is_empty() {
        println!("Successfully killed process with PID {}", &pid);
        println!("Used force: {}", args.force);
    }
}
