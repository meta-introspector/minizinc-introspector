use serde::Deserialize;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub project_root: PathBuf,
    pub github_root: PathBuf,
    pub home_dir: PathBuf,
    pub build_target: String, // Added build_target
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = PathBuf::from("config.toml");
        let config_content = fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
}