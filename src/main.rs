mod client;
mod server;

use clap::{Parser, Subcommand};

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
    }
}

