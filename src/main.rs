use clap::{Parser, Subcommand};

mod core;
mod provider;
mod command;
use command::show;
use command::lock;


#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Show {},
    Lock {},
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command to run
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Show {}) => {
            show::show();
        }
        Some(Commands::Lock {}) => {
            lock::lock();
        }
        None => println!("No command provided"),
    }
}
 