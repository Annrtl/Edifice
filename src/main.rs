use clap::{Parser, Subcommand};

mod core;
mod provider;
use command::fetch;
mod command;
use command::show;
use command::lock;


#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Show {},
    Lock {},
    Fetch {},
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
        Some(Commands::Fetch {}) => {
            match fetch::fetch() {
                Ok(_) => println!("Done"),
                Err(err) => println!("Command failed: {err}"),
            };
        }
        None => println!("No command provided"),
    }
}
 