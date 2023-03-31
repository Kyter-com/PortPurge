use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'n', long = "name")]
    name: String,

    /// Port number to kill process on
    #[arg(short = 'p', long = "port")]
    port: u32,
}

fn main() {
    println!("Running portassassin");
    let args = Args::parse();

    println!("Hello {}", args.name);
    println!("Port to kill: {}", args.port);
}
