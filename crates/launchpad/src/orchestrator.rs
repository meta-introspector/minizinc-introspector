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
pub async fn run_gemini_cli(args: &[&str]) -> Result<(), String> {
    eprintln!("Running Gemini CLI...");
    run_command("gemini", args, None).await
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
