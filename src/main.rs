use std::path::PathBuf;

use clap::{Parser, ValueEnum, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(long)]
    host: Option<PathBuf>,

    #[arg(long)]
    port: Option<PathBuf>,

    #[command(subcommand)]
    command: ModeSelectCommand
}

#[derive(Subcommand)]
enum ModeSelectCommand {
    Server {
        #[command(subcommand)]
        command: ServerSubcommand
    },

    Client {
        #[command(subcommand)]
        command: ClientSubcommand
    }
}

#[derive(Subcommand)]
enum ClientSubcommand {
    /// Adds an object
    Add {
        object: ClientObject,
    },

    /// Removes an object
    Remove {
        /// Which type of object to remove
        object: ClientObject,
        name: String
    },

    /// Lists objects
    List {
        /// List all, even inactive/disabled
        object: ClientObject,

        #[arg(short, long)]
        all: Option<bool> 
    },

    /// Edits an object
    Edit {
        object: ClientObject
    },

    // Done messing with objects

    /// Connect to server
    Connect {
        name: String
    }
}

#[derive(Subcommand)]
enum ServerSubcommand {
    /// Adds an object
    Add {
        object: ClientObject,
    },

    /// Removes an object
    Remove {
        /// Which type of object to remove
        object: ClientObject,
        name: String
    },

    /// Lists objects
    List {
        /// List all, even inactive/disabled
        object: ClientObject,

        #[arg(short, long)]
        all: Option<bool> 
    },

    /// Edits an object
    Edit {
        object: ClientObject
    },

    /// Edits the config
    Config {
        setting: ClientSetting,
        value: Option<String>
    },
    
    /// Connect to server
    Connect {
        name: String
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ClientObject {
    Nic,
    Account,
    Cert,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ClientSetting {
    Remote
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ServerObject {
    Hub,
    User,
    Cert,
    Table,
    Group,

}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ServerSetting {
    DHCP,
    IPSec,
    Log,

}

fn main() {
    let cli = Cli::parse();

    // Continued program logic goes here...
    match cli.command {
        ModeSelectCommand::Client { command } => {
            match command {
                ClientSubcommand::Add { object } => todo!(),
                ClientSubcommand::Remove { object, name } => todo!(),
                ClientSubcommand::List { object, all } => todo!(),
                ClientSubcommand::Edit { object } => todo!(),
                ClientSubcommand::Connect { name } => todo!(),
            }
        }
        ModeSelectCommand::Server { command } => todo!(),
    }
}