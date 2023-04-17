use clap::Parser;
use std::env::consts::OS;

mod utils;

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
    let args = Args::parse();

    if OS == "windows" {
        let pid_result = utils::windows_find_pid_on_port(args.port);
        let pid = match pid_result {
            Ok(Some(pid)) => pid,
            Ok(None) => {
                println!("No processes running on port {}", args.port);
                std::process::exit(0);
            }
            Err(error) => {
                if error.contains("Failed to find PID on port") {
                    println!("No processes running on port {}", args.port);
                    std::process::exit(0);
                }
                println!("Error: {}", error);
                std::process::exit(1);
            }
        };

        let kill_result = utils::windows_kill_process_with_pid(&pid, args.force);
        match kill_result {
            Ok(_) => {
                println!(
                    "Successfully killed process on port {} with PID {}",
                    args.port, &pid
                );
                std::process::exit(0);
            }
            Err(error) => {
                println!("Error: {}", error);
                std::process::exit(1);
            }
        }
    }

    let pid_result = utils::unix_find_pid_on_port(args.port);
    let pid = match pid_result {
        Ok(Some(pid)) => pid,
        Ok(None) => {
            println!("No processes running on port {}", args.port);
            std::process::exit(0);
        }
        Err(error) => {
            println!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let kill_result = utils::unix_kill_process_with_pid(&pid, args.force);
    match kill_result {
        Ok(_) => println!(
            "Successfully killed process on port {} with PID {}",
            args.port, &pid
        ),
        Err(error) => {
            println!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
