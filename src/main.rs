use clap::Parser;

mod environment;
use crate::environment::{Environment, Befunge93Interpreter};

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
    #[arg(short, long, default_value = "false")]
    quiet: bool,
}

fn main() {
    let args = Args::parse();
    let file_contents = std::fs::read_to_string(args.path)
        .expect("Could not read file");
    let mut env = Environment::new(file_contents);
    env.execute();
    if !args.quiet {
        println!("\nExecution Finished.")
    }
}

