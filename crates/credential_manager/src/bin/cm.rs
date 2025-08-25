use clap::{Parser, Subcommand};
use credential_manager::{store_credential, retrieve_credential};
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

async fn handle_store_command(service: &str, username: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let password = rpassword::read_password()?;
    store_credential(service, username, &password)?;
    println!("Credential stored successfully.");
    Ok(())
}

async fn handle_retrieve_command(service: &str, username: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let password = retrieve_credential(service, username)?;
    println!("Retrieved credential: {}", password);
    Ok(())
}

async fn handle_list_command() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Listing credentials is not directly supported by the keyring crate.");
    println!("You can try retrieving specific credentials if you know the service and username.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Store { service, username } => {
            handle_store_command(service, username).await?;
        },
        Commands::Retrieve { service, username } => {
            handle_retrieve_command(service, username).await?;
        },
        Commands::List => {
            handle_list_command().await?;
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