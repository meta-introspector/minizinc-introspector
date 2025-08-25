use clap::{Parser, Subcommand};
use credential_manager::{store_credential, retrieve_credential};
use rpassword::read_password;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Store { service, username } => {
            println!("Enter password for service '{}' and username '{}':", service, username);
            let password = read_password()?;
            store_credential(service, username, &password)?;
            println!("Credential stored successfully.");
        },
        Commands::Retrieve { service, username } => {
            match retrieve_credential(service, username) {
                Ok(password) => {
                    println!("Password for service '{}' and username '{}': {}", service, username, password);
                },
                Err(e) => {
                    eprintln!("Error retrieving credential: {}", e);
                }
            }
        },
        Commands::List => {
            println!("Listing credentials is not directly supported by the keyring crate.");
            println!("You can try retrieving specific credentials if you know the service and username.");
        },
    }

    Ok(())
}
