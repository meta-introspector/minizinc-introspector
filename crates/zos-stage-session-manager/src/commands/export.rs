use clap::Parser;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ExportArgs {
    /// Number of Gemini instances to launch
    #[arg(short, long, default_value_t = 1)]
    pub gemini_instances: u8,

    /// Whether to record the session with asciinema
    #[arg(short, long, default_value_t = false)]
    pub record_session: bool,

    /// Launch a single Gemini instance in the background, detached from the current terminal
    #[arg(long, default_value_t = false)]
    pub background_detached: bool,

    /// Output file path for the session configuration
    #[arg(short, long, default_value = "sessions/last_session.json")]
    pub output: String,
}

pub fn handle_export_command(args: &ExportArgs) -> Result<(), String> {
    eprintln!("Exporting session configuration to: {}", args.output);
    let config = ExportArgs {
        gemini_instances: args.gemini_instances,
        record_session: args.record_session,
        background_detached: args.background_detached, // Include in export
        output: args.output.clone(),
    };
    let json_config = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config to JSON: {e}"))?;

    let output_path = PathBuf::from(&args.output);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory {:?}: {e}", parent))?;
    }

    fs::write(&output_path, json_config)
        .map_err(|e| format!("Failed to write config to file {:?}: {e}", output_path))?;

    eprintln!("Session configuration exported successfully.");
    Ok(())
}
