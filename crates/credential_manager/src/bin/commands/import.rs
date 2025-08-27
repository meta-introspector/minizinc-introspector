use clap::Subcommand;
use crate::{store_credential};
use std::fs;
use std::path::PathBuf;
use serde_yaml;
use serde::Deserialize;
use std::collections::HashMap;
use std::env; // For std::env::var("HOME")
use ini::configparser::ini::Ini;

#[derive(Subcommand, Debug)]
pub enum ImportService {
    /// Imports credentials from AWS configuration
    Aws,
    /// Imports credentials from GitHub CLI configuration
    Github,
    /// Imports credentials from Gemini CLI configuration
    GeminiCli,
}

pub async fn handle_import_command(service: &ImportService) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    match service {
        ImportService::Aws => {
            println!("Importing AWS credentials...");
            let home_dir = PathBuf::from(env::var("HOME")?);
            let aws_credentials_path = home_dir.join(".aws").join("credentials");
            let aws_config_path = home_dir.join(".aws").join("config");

            // Process credentials file
            if aws_credentials_path.exists() {
                let mut config = Ini::new();
                let creds = config.load(&aws_credentials_path.to_string_lossy())?;
                for (section, props) in creds.iter() {
                    if let Some(access_key) = props.get("aws_access_key_id") {
                        store_credential("aws", &format!("{}_access_key_id", section), access_key.as_deref().unwrap_or_default())?;
                        println!("Stored AWS access key for profile: {}", section);
                    }
                    if let Some(secret_key) = props.get("aws_secret_access_key") {
                        store_credential("aws", &format!("{}_secret_access_key", section), secret_key.as_deref().unwrap_or_default())?;
                        println!("Stored AWS secret key for profile: {}", section);
                    }
                    if let Some(session_token) = props.get("aws_session_token") {
                        store_credential("aws", &format!("{}_session_token", section), session_token.as_deref().unwrap_or_default())?;
                        println!("Stored AWS session token for profile: {}", section);
                    }
                }
            } else {
                println!("AWS credentials file not found at: {:?}", aws_credentials_path);
            }

            // Manual parsing for config file (simplified, only for demonstration)
            if aws_config_path.exists() {
                let mut config = Ini::new();
                let config = config.load(&aws_config_path.to_string_lossy())?;
                for (section, props) in config.iter() {
                    // Example: store region if needed, but not directly a credential
                    if let Some(region) = props.get("region") {
                        println!("Found AWS region for profile {}: {}", section, region.as_deref().unwrap_or_default());
                    }
                }
            } else {
                println!("AWS config file not found at: {:?}", aws_config_path);
            }
        },
        ImportService::Github => {
            println!("Importing GitHub credentials...");
            #[derive(Debug, Deserialize)]
            struct GhHosts {
                #[serde(flatten)]
                hosts: HashMap<String, GhHostEntry>,
            }

            #[derive(Debug, Deserialize)]
            struct GhHostEntry {
                oauth_token: String,
                // Add other fields if needed, e.g., user, git_protocol
            }

            let gh_config_path = PathBuf::from(env::var("HOME")?)
                .join(".config")
                .join("gh")
                .join("hosts.yml");

            if !gh_config_path.exists() {
                println!("GitHub hosts file not found at: {:?}", gh_config_path);
                return Ok(());
            }

            let contents = fs::read_to_string(&gh_config_path)?;
            let gh_hosts: GhHosts = serde_yaml::from_str(&contents)?;

            for (hostname, entry) in gh_hosts.hosts.into_iter() {
                store_credential(hostname.as_str(), "gh_oauth_token", &entry.oauth_token)?;
                println!("Stored GitHub token for host: {}", hostname);
            }
        },
        ImportService::GeminiCli => {
            println!("Importing Gemini CLI credentials...");
            println!("Please enter your Gemini API Key:");
            let api_key = rpassword::read_password()?;
            store_credential("gemini_cli", "default", &api_key)?;
            println!("Gemini CLI API Key stored successfully.");
        },
    }
    Ok(())
}