use clap::{Parser, Subcommand};

use hydra::command::check;
use hydra::command::fetch;
use hydra::command::show;
use hydra::command::update;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Show {},
    Check {},
    Update {},
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
            match show::show() {
                Ok(_) => println!("Done"),
                Err(err) => println!("Command failed: {err}"),
            };
        }
        Some(Commands::Check {}) => {
            match check::check() {
                Ok(_) => println!("Done"),
                Err(err) => println!("Command failed: {err}"),
            };
        }
        Some(Commands::Update {}) => {
            match update::update() {
                Ok(_) => println!("Done"),
                Err(err) => println!("Command failed: {err}"),
            };
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
