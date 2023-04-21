use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Port number to kill process on
    #[arg(short = 'p', long = "port")]
    port: u32,

    /// Kill the process without giving it an opportunity to clean up (unix)
    #[arg(short = 'f', long = "force")]
    force: bool,
}

// Write a function that returns the current OS
// and use it to determine which code to run

fn main() {
    let args = Args::parse();

    #[cfg(target_family = "windows")]
    {
        let pid_result = utils::windows_find_pids_on_port(args.port);
        let pids = match pid_result {
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

        for pid in pids {
            let kill_result = utils::windows_kill_process_with_pid(&pid);
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
    }

    #[cfg(target_family = "unix")]
    {
        let pid_result = utils::unix_find_pids_on_port(args.port);
        let pids = match pid_result {
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

        for pid in pids {
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
    }
}
