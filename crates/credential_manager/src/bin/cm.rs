use clap::{Parser, Subcommand};
use credential_manager::{store_credential, retrieve_credential};
use rpassword::read_password;

// Removed extern crate declarations

mod commands;
use commands::auth::{AuthService, handle_auth_command};
use commands::import::{ImportService, handle_import_command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Stores a credential
    Store {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
    },
    /// Retrieves a credential
    Retrieve {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
    },
    /// Lists stored credentials (placeholder - keyring does not directly support listing)
    List,
    /// Authenticates with a service using OAuth2
    Auth {
        #[command(subcommand)]
        service: AuthService,
    },
    /// Imports credentials from various sources
    Import {
        #[command(subcommand)]
        service: ImportService,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Store { service, username } => {
            let password = rpassword::read_password()?;
            store_credential(service, username, &password)?;
            println!("Credential stored successfully.");
        },
        Commands::Retrieve { service, username } => {
            let password = retrieve_credential(service, username)?;
            println!("Retrieved credential: {}", password);
        },
        Commands::List => {
            println!("Listing credentials is not directly supported by the keyring crate.");
            println!("You can try retrieving specific credentials if you know the service and username.");
        },
        Commands::Auth { service } => {
            handle_auth_command(service).await?;
        },
        Commands::Import { service } => {
            handle_import_command(service).await?;
        },
    }
    Ok(())
}