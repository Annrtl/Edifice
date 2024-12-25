use clap::{Parser, Subcommand};

use edifice::command::add;
use edifice::command::check;
use edifice::command::fetch;
use edifice::command::install;
use edifice::command::list;
use edifice::command::info;
use edifice::command::update;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Add {
        module: Option<String>,
        #[arg(short, long)]
        dry: bool,
    },
    Check {},
    Fetch {},
    Install {},
    List {
        pattern: Option<String>,
    },
    Info {},
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
        Some(Commands::Add { module, dry }) => {
            let _module = match module {
                Some(module) => module,
                None => return Err("No module provided".to_string()),
            };
            match add::add(_module, dry) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
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
        Some(Commands::List {pattern}) => {
            match list::list(pattern) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        Some(Commands::Info {}) => {
            match info::info() {
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
