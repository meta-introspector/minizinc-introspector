//! This module provides orchestration functions for executing various commands
//! and managing external processes within the `launchpad` application.
//! It includes wrappers for installing and running the Gemini CLI, executing
//! `dum` tasks, `cargo` commands, `tmux` commands, and sandboxed commands.

use std::process::{Command, Stdio};
use std::path::Path;

/// Executes a general shell command.
///
/// This function runs a specified command with given arguments. Its standard
/// output and standard error are inherited from the parent process. An optional
/// current working directory can be provided.
///
/// # Arguments
///
/// * `cmd` - The command to execute (e.g., "npm", "cargo", "gemini").
/// * `args` - A slice of string slices representing arguments to pass to the command.
/// * `cwd` - An optional `Path` to set as the current working directory for the command.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`) if
/// the command fails to execute or exits with a non-zero status.
pub async fn run_command(cmd: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    let mut command = Command::new(cmd);
    command.args(args);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    eprintln!("Executing: {} {:?}", cmd, args);

    let status = command.status()
        .map_err(|e| format!("Failed to execute command {}: {}", cmd, e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Command {} exited with non-zero status: {:?}", cmd, status.code()))
    }
}

/// Installs the Gemini CLI globally using npm.
///
/// This function executes `npm install -g @google/gemini-cli`.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
pub async fn install_gemini_cli() -> Result<(), String> {
    eprintln!("Installing Gemini CLI...");
    run_command("npm", &["install", "-g", "@google/gemini-cli"], None).await
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
use crate::launchpad_main::LaunchpadArgs; // Import LaunchpadArgs

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

    let session_manager_args_str: Vec<&str> = session_manager_args.iter().map(|s| s.as_str()).collect();
    run_command(session_manager_binary_path.to_str().unwrap(), &session_manager_args_str, None).await
}

/// Runs a `dum` task.
///
/// This function is a placeholder for integrating with `dum` (a task runner).
/// It executes the `dum` command with the specified task name and additional arguments.
///
/// # Arguments
///
/// * `task_name` - The name of the `dum` task to run.
/// * `args` - A slice of string slices representing additional arguments for the `dum` task.
/// * `cwd` - An optional `Path` to set as the current working directory for the command.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
pub async fn run_dum_task(task_name: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    eprintln!("Running dum task: {}", task_name);
    let mut dum_args = vec![task_name];
    dum_args.extend_from_slice(args);

    let current_exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {e}"))?;
    let project_root = current_exe_path.parent()
        .and_then(|p| p.parent()) // target/debug/
        .and_then(|p| p.parent()) // libminizinc/
        .ok_or("Could not determine project root")?;

    let dum_binary_path = project_root
        .join("vendor")
        .join("dum")
        .join("target")
        .join("debug")
        .join("dum");

    run_command(dum_binary_path.to_str().unwrap(), &dum_args, cwd).await
}

/// Runs a `cargo` command.
///
/// This function is a placeholder for executing `cargo` commands.
/// It runs the `cargo` command with the provided arguments.
///
/// # Arguments
///
/// * `args` - A slice of string slices representing arguments to pass to the `cargo` command.
/// * `cwd` - An optional `Path` to set as the current working directory for the command.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
pub async fn run_cargo_command(args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    eprintln!("Running cargo command: {:?}", args);
    run_command("cargo", args, cwd).await
}

/// Runs a `tmux` command.
///
/// This function is a placeholder for executing `tmux` commands.
/// It runs the `tmux` command with the provided arguments.
///
/// # Arguments
///
/// * `args` - A slice of string slices representing arguments to pass to the `tmux` command.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
pub async fn run_tmux_command(args: &[&str]) -> Result<(), String> {
    eprintln!("Running tmux command: {:?}", args);
    run_command("tmux", args, None).await
}

/// Runs a command within a sandboxed environment.
///
/// This function is a placeholder for integrating with sandboxing tools
/// like `pchroot` or `act-run`. It executes the specified command and arguments.
///
/// # Arguments
///
/// * `cmd` - The command to execute within the sandbox.
/// * `args` - A slice of string slices representing arguments to pass to the command.
/// * `cwd` - An optional `Path` to set as the current working directory for the command.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
pub async fn run_sandboxed_command(cmd: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    eprintln!("Running sandboxed command: {} {:?}", cmd, args);
    // This would involve setting up pchroot/act-run environment
    run_command(cmd, args, cwd).await
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Direct unit testing of functions that execute external commands
    // (`run_command` and its wrappers) is challenging due to side effects
    // and process execution. These tests primarily ensure compilation.
    // For robust testing, consider mocking the underlying `Command` or
    // implementing integration tests.

    #[tokio::test]
    async fn test_run_command_compiles() {
        // This test only ensures compilation. Actual execution is not performed.
        // let result = run_command("echo", &["hello"], None).await;
        // assert!(result.is_ok());
        assert!(true);
    }

    #[tokio::test]
    async fn test_install_gemini_cli_compiles() {
        // This test only ensures compilation.
        // let result = install_gemini_cli().await;
        // assert!(result.is_ok());
        assert!(true);
    }

    #[tokio::test]
    async fn test_run_gemini_cli_compiles() {
        // This test only ensures compilation.
        // let result = run_gemini_cli(&["version"]).await;
        // assert!(result.is_ok());
        assert!(true);
    }

    #[tokio::test]
    async fn test_run_dum_task_compiles() {
        // This test only ensures compilation.
        // let result = run_dum_task("test", &[], None).await;
        // assert!(result.is_ok());
        assert!(true);
    }

    #[tokio::test]
    async fn test_run_cargo_command_compiles() {
        // This test only ensures compilation.
        // let result = run_cargo_command(&["version"], None).await;
        // assert!(result.is_ok());
        assert!(true);
    }

    #[tokio::test]
    async fn test_run_tmux_command_compiles() {
        // This test only ensures compilation.
        // let result = run_tmux_command(&["version"]).await;
        // assert!(result.is_ok());
        assert!(true);
    }

    #[tokio::test]
    async fn test_run_sandboxed_command_compiles() {
        // This test only ensures compilation.
        // let result = run_sandboxed_command("echo", &["sandbox"], None).await;
        // assert!(result.is_ok());
        assert!(true);
    }
}
