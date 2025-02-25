use clap::{Parser, Subcommand};

use edifice::command::add;
use edifice::command::build;
use edifice::command::check;
use edifice::command::fetch;
use edifice::command::info;
use edifice::command::install;
use edifice::command::list;
use edifice::command::update;

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Add {
        module: Option<String>,
        #[arg(short, long)]
        dry: bool,
    },
    Build {},
    Check {},
    Fetch {},
    Info {},
    Install {},
    List {
        pattern: Option<String>,
    },
    New {
        name: Option<String>,
    },
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
        Some(Commands::Build {}) => {
            match build::build() {
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
        Some(Commands::Info {}) => {
            match info::info() {
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
        Some(Commands::List { pattern }) => {
            match list::list(pattern) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
        Some(Commands::New { name }) => {
            match name {
                Some(name) => {
                    match edifice::command::new::new(name) {
                        Ok(_) => return Ok(()),
                        Err(err) => return Err(err),
                    };
                }
                None => return Err("No name provided".to_string()),
            }
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
