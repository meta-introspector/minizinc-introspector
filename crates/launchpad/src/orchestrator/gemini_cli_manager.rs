use std::path::Path;

use crate::launchpad_main::LaunchpadArgs;
use super::command_executor;

/// Installs the Gemini CLI globally using npm.
///
/// This function executes `npm install -g @google/gemini-cli`.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
pub async fn install_gemini_cli() -> Result<(), String> {
    eprintln!("Installing Gemini CLI...");
    command_executor::run_command("npm", &["install", "-g", "@google/gemini-cli"], None).await
}

/// Runs a Gemini CLI command.
///
/// This function executes the `gemini` command with the provided arguments.
///
/// # Arguments
///
/// * `args` - A slice of string slices representing arguments to pass to the `gemini` command.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
pub async fn run_gemini_cli(args: &LaunchpadArgs) -> Result<(), String> {
    eprintln!("Running Gemini CLI via session manager...");

    let current_exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {e}"))?;
    let project_root = current_exe_path.parent()
        .and_then(|p| p.parent()) // target/debug/
        .and_then(|p| p.parent()) // libminizinc/
        .ok_or("Could not determine project root")?;

    let session_manager_binary_path = project_root
        .join("target")
        .join("debug")
        .join("zos-stage-session-manager");

    let mut session_manager_args = vec!["Launch".to_string()];

    // Pass other relevant Gemini CLI options to the session manager if it supports them
    if let Some(model) = &args.model {
        session_manager_args.push("--model".to_string());
        session_manager_args.push(model.clone());
    }
    if let Some(prompt) = &args.prompt {
        session_manager_args.push("--prompt".to_string());
        session_manager_args.push(prompt.clone());
    }
    if let Some(prompt_interactive) = &args.prompt_interactive {
        session_manager_args.push("--prompt-interactive".to_string());
        session_manager_args.push(prompt_interactive.clone());
    }
    if args.sandbox.unwrap_or(false) {
        session_manager_args.push("--sandbox".to_string());
    }
    if let Some(sandbox_image) = &args.sandbox_image {
        session_manager_args.push("--sandbox-image".to_string());
        session_manager_args.push(sandbox_image.clone());
    }
    if args.debug {
        session_manager_args.push("--debug".to_string());
    }
    if args.all_files {
        session_manager_args.push("--all-files".to_string());
    }
    if args.show_memory_usage {
        session_manager_args.push("--show-memory-usage".to_string());
    }
    if args.yolo {
        session_manager_args.push("--yolo".to_string());
    }
    if let Some(approval_mode) = &args.approval_mode {
        session_manager_args.push("--approval-mode".to_string());
        let approval_mode_str = approval_mode.to_string();
        session_manager_args.push(approval_mode_str);
    }
    if args.telemetry.unwrap_or(false) {
        session_manager_args.push("--telemetry".to_string());
    }
    if let Some(telemetry_target) = &args.telemetry_target {
        session_manager_args.push("--telemetry-target".to_string());
        let telemetry_target_str = telemetry_target.to_string();
        session_manager_args.push(telemetry_target_str);
    }
    if let Some(telemetry_otlp_endpoint) = &args.telemetry_otlp_endpoint {
        session_manager_args.push("--telemetry-otlp-endpoint".to_string());
        session_manager_args.push(telemetry_otlp_endpoint.clone());
    }
    if args.telemetry_log_prompts.unwrap_or(false) {
        session_manager_args.push("--telemetry-log-prompts".to_string());
    }
    if let Some(telemetry_outfile) = &args.telemetry_outfile {
        session_manager_args.push("--telemetry-outfile".to_string());
        session_manager_args.push(telemetry_outfile.clone());
    }
    if args.checkpointing {
        session_manager_args.push("--checkpointing".to_string());
    }
    if args.experimental_acp.unwrap_or(false) {
        session_manager_args.push("--experimental-acp".to_string());
    }
    for name in &args.allowed_mcp_server_names {
        session_manager_args.push("--allowed-mcp-server-names".to_string());
        session_manager_args.push(name.clone());
    }
    for extension in &args.extensions {
        session_manager_args.push("--extensions".to_string());
        session_manager_args.push(extension.clone());
    }
    if args.list_extensions.unwrap_or(false) {
        session_manager_args.push("--list-extensions".to_string());
    }
    if let Some(proxy) = &args.proxy {
        session_manager_args.push("--proxy".to_string());
        session_manager_args.push(proxy.clone());
    }
    for dir in &args.include_directories {
        session_manager_args.push("--include-directories".to_string());
        session_manager_args.push(dir.clone());
    }
    if args.version.unwrap_or(false) {
        session_manager_args.push("--version".to_string());
    }
    if args.help.unwrap_or(false) {
        session_manager_args.push("--help".to_string());
    }

    // Add custom arguments for the CRQ workflow
    if let Some(crq) = &args.crq {
        session_manager_args.push("--crq".to_string());
        session_manager_args.push(crq.clone());
    }
    if let Some(mode) = &args.mode {
        session_manager_args.push("--mode".to_string());
        session_manager_args.push(mode.clone());
    }
    if let Some(inside) = &args.inside {
        session_manager_args.push("--inside".to_string());
        session_manager_args.push(inside.clone());
    }
    if let Some(via) = &args.via {
        session_manager_args.push("--via".to_string());
        session_manager_args.push(via.clone());
    }
    if let Some(crq_path) = &args.crq_path {
        session_manager_args.push("--crq-path".to_string());
        session_manager_args.push(crq_path.clone());
    }
    if let Some(target_repo_url) = &args.target_repo_url {
        session_manager_args.push("--target-repo-url".to_string());
        session_manager_args.push(target_repo_url.clone());
    }
    if let Some(workflow_file_in_repo) = &args.workflow_file_in_repo {
        session_manager_args.push("--workflow-file-in-repo".to_string());
        session_manager_args.push(workflow_file_in_repo.clone());
    }

    let session_manager_args_str: Vec<&str> = session_manager_args.iter().map(|s| s.as_str()).collect();
    command_executor::run_command(session_manager_binary_path.to_str().unwrap(), &session_manager_args_str, None).await
}
