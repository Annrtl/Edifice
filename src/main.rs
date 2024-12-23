use clap::{Parser, Subcommand};

use hydra::command::add;
use hydra::command::check;
use hydra::command::fetch;
use hydra::command::install;
use hydra::command::list;
use hydra::command::show;
use hydra::command::update;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Add {
        module: Option<String>,
    },
    Check {},
    Fetch {},
    Install {},
    List {},
    Show {},
    Update {},
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command to run
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Add { module }) => {
            match module {
                Some(module) => match add::add(module) {
                    Ok(_) => return Ok(()),
                    Err(err) => return Err(err),
                },
                None => return Err("No module provided".to_string()),
            };
        }
        Some(Commands::Check {}) => {
            match check::check() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        Some(Commands::Fetch {}) => {
            match fetch::fetch() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        Some(Commands::Install {}) => {
            match install::install() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        Some(Commands::List {}) => {
            match list::list() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        Some(Commands::Show {}) => {
            match show::show() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        Some(Commands::Update {}) => {
            match update::update() {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        None => return Err("No command provided".to_string()),
    }
}
