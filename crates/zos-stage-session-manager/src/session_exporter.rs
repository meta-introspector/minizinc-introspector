use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs;

use crate::args::LaunchArgs;

pub async fn export_session_config(args: &LaunchArgs) -> Result<(), String> {
    eprintln!("Exporting session configuration to: {}", args.output);
    let config = LaunchArgs {
        gemini_instances: args.gemini_instances,
        record_session: args.record_session,
        background_detached: args.background_detached, // Include in export
        // All other fields need to be explicitly initialized or handled
        // For now, we'll just use default values or None for simplicity
        model: None,
        prompt: None,
        prompt_interactive: None,
        sandbox: None,
        sandbox_image: None,
        debug: false,
        all_files: false,
        show_memory_usage: false,
        yolo: false,
        approval_mode: None,
        telemetry: None,
        telemetry_target: None,
        telemetry_otlp_endpoint: None,
        telemetry_log_prompts: None,
        telemetry_outfile: None,
        checkpointing: false,
        experimental_acp: None,
        allowed_mcp_server_names: Vec::new(),
        extensions: Vec::new(),
        list_extensions: None,
        proxy: None,
        include_directories: Vec::new(),
        version: None,
        help: None,
        crq: None,
        mode: None,
        inside: None,
        via: None,
        crq_path: None,
        target_repo_url: None,
        workflow_file_in_repo: None,
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
