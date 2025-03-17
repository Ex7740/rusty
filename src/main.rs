mod client;
mod server;

use clap::{Parser, Subcommand};
use std::process::Command;
use std::fs;

/// Rusty CLI Tool
#[derive(Parser)]
#[command(name = "rusty")]
#[command(version = "1.0")]
#[command(about = "Rusty CLI Tool with client and server commands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Client operations
    Client {
        #[command(subcommand)]
        subcommand: ClientCommands,
    },
    /// Server operations
    Server {
        #[command(subcommand)]
        subcommand: ServerCommands,
    },
    /// Update and install the application
    Update,
}

#[derive(Subcommand)]
enum ClientCommands {
    /// Connect to a server
    Connect {
        /// The address to connect to
        address: String,
    },
}

#[derive(Subcommand)]
enum ServerCommands {
    /// Start the server
    Start,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Client { subcommand } => match subcommand {
            ClientCommands::Connect { address } => {
                if let Err(e) = client::connect_to_server(address) {
                    eprintln!("Client error: {}", e);
                }
            }
        },
        Commands::Server { subcommand } => match subcommand {
            ServerCommands::Start => {
                if let Err(e) = server::start_server() {
                    eprintln!("Server error: {}", e);
                }
            }
        },
        Commands::Update => {
            let build_status = Command::new("cargo")
                .args(&["build", "--release"])
                .status()
                .expect("Failed to build the project");

            if build_status.success() {
                println!("Build successful. Moving executable to Program Files.");

                let exe_source = "target/release/rusty.exe";
                let exe_destination = "C:/Program Files/rusty/rusty.exe";

                if let Err(e) = fs::create_dir_all("C:/Program Files/rusty") {
                    eprintln!("Failed to create destination directory: {}", e);
                }

                if let Err(e) = fs::copy(exe_source, exe_destination) {
                    eprintln!("Failed to move executable: {}", e);
                } else {
                    println!("Executable moved successfully.");

                    if let Err(e) = fs::remove_dir_all("target") {
                        eprintln!("Failed to delete target folder: {}", e);
                    } else {
                        println!("Target folder deleted successfully.");
                    }
                }
            } else {
                eprintln!("Build failed.");
            }
        }
    }
}

