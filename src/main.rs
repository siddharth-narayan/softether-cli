use core::time;
use std::{alloc::System, env, io::{Error, Write}, os::unix::process, path::PathBuf, process::{Command, Stdio}, thread::sleep};

use clap::{Parser, ValueEnum, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    port: Option<u16>,

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
        name: Option<String>
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
        object: ClientObject,
        name: String
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
        name: Option<String>
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
        object: ClientObject,
        name: String
    },

    /// Edits the config
    Config {
        setting: Option<ClientSetting>,
        value: Option<String>
    },
    
    /// Connect to server
    Connect {
        account_name: String
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

const VPNCMD: &str = "/nix/store/406yxwg58v2vyrl6rg9jqag7s3jjggyz-softether-5.02.5183/bin/vpncmd";
fn main() {
    let cli = Cli::parse();
    let host = cli.host.clone().unwrap_or("localhost".to_string());
    let port = cli.port.unwrap_or(443);

    match cli.command {
        ModeSelectCommand::Client { command } => {
            match command {
                ClientSubcommand::Add { object, name} => client_add(object, name, host, port),
                ClientSubcommand::Remove { object, name } => todo!(),
                ClientSubcommand::List { object, all } => todo!(),
                ClientSubcommand::Edit { object, name} => todo!(),
                ClientSubcommand::Connect { name } => todo!(),
            }
        }
        ModeSelectCommand::Server { command } => todo!(),
    }
}

fn client_add(object: ClientObject, name: Option<String>, host: String, port: u16) {
    match object {
        ClientObject::Nic => {
            if name.is_none() && env::consts::OS != "windows" { 
                println!("You must specify the name of the NIC that will be added!");
            } else {
                
                let res = Command::new(VPNCMD)
                    .arg(host.clone())
                    .arg("/CLIENT".to_string())
                    .arg("/CMD".to_string())
                    .arg("NicCreate")
                    .arg(name.clone().unwrap_or("".to_string()))
                    .stdout(Stdio::null())
                    .status().expect("Couldn't run the command");

                println!("Running command vpncmd {host} /CLIENT /CMD NicCreate {}", name.unwrap_or("".to_string()));
                println!("Process exited with status {res}");
            }
        },
        ClientObject::Account => {
            // Placeholder
            let setting_name = b"VPN\n";
            let account_name = b"Siddharth\n";
            let server_host = b"novaphaze.com:992\n";
            let password = b"password\n";
            let hub = b"VPN\n";
            let nic = b"VPN\n";

            let mut child = Command::new(VPNCMD)
                .arg(host)
                .arg("/CLIENT".to_string())
                .arg("/CMD".to_string())
                .stdin(Stdio::piped()).spawn().unwrap();
            
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(b"AccountCreate\n");
            stdin.write_all(setting_name);
            stdin.write_all(server_host);
            stdin.write_all(hub);
            stdin.write_all(account_name);
            stdin.write_all(nic);

            stdin.write_all(b"AccountPasswordSet\n\n");
            stdin.write_all(setting_name);
            stdin.write_all(password);
            stdin.write_all(password);

            // IDK why this is needed
            sleep(time::Duration::from_secs(1));
            stdin.write_all(b"standard\n").unwrap();
            stdin.write_all(b"exit\n").unwrap();

            child.wait();
            
        },
        ClientObject::Cert => todo!(),
    }
}