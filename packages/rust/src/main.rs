use clap::Parser;
use std::process::Command;
use std::process::Stdio;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Port number to kill process on
    #[arg(short = 'p', long = "port")]
    port: u32,
}

fn main() {
    println!("Running portassassin");
    let args = Args::parse();

    println!("Port to kill: {}", args.port);

    let processes = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}", args.port))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    println!(
        "{:?}",
        String::from_utf8_lossy(&processes.wait_with_output().unwrap().to_owned().stdout)
    );
}
