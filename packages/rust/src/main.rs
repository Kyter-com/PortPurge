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
}

fn main() {
    println!("Running PortAssassin");
    let args = Args::parse();

    println!("Port to kill: {}", args.port);

    let command = Command::new("lsof")
        .arg(format!("-i:{}", args.port))
        .arg("-t")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command")
        .wait_with_output()
        .expect("Failed to wait for command");

    let mut pid_string = String::from_utf8(command.stdout).expect("Failed to get PID");

    // Trim the trailing \n of the PID string
    trim_newline(&mut pid_string);

    if pid_string.is_empty() {
        println!("No process running on port {}", args.port);
        std::process::exit(0);
    }

    let pid: i32 = pid_string.parse().unwrap_or(0);

    println!("Found process running with PID: {}", pid);
}
