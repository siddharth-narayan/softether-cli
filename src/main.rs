use std::path::PathBuf;

use clap::{Parser, ValueEnum, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(long)]
    host: Option<PathBuf>,

    #[arg(long)]
    port: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[clap(required = true)]
    mode: Mode,

    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    /// Adds an object
    Add {
        object: Object,
    },

    /// Removes an object
    Remove {
        /// Which type of object to remove
        object: Object,
        name: String
    },

    /// Lists objects
    List {
        /// List all, even inactive/disabled
        object: Object,

        #[arg(short, long)]
        all: Option<bool> 
    },

    /// Edits an object
    Edit {
        object: Object
    },

    // Done messing with objects

    /// Connect to server
    Connect {
        name: String
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Server,
    Client
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Object {
    Nic,
    User,
    Hub,
}

fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.verbose {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // Continued program logic goes here...
}