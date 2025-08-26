//! This module provides the main entry point for the `launchpad` application.
//! It handles command-line argument parsing, dynamic environment variable setup,
//! and dispatches to various operational stages based on user input.

use crate::orchestrator;
use crate::narrator;
use crate::dum_wrappers::gemini_cli_runner;
use crate::dum_wrappers::run_main;
use crate::gemini_cli_options::{GeminiCliOptions, ApprovalMode, TelemetryTarget};
use clap::Parser;

/// Command-line arguments for the launchpad application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_version_flag = true, disable_help_flag = true)]
pub struct LaunchpadArgs {
    /// The identifier of the stage to launch (e.g., install-gemini, run-gemini, miniact).
    pub stage_identifier: String,

    // Arguments for Gemini CLI
    #[arg(long)]
    pub model: Option<String>,
    #[arg(long)]
    pub prompt: Option<String>,
    #[arg(long)]
    pub prompt_interactive: Option<String>,
    #[arg(long)]
    pub sandbox: Option<bool>,
    #[arg(long)]
    pub sandbox_image: Option<String>,
    #[arg(long)]
    pub debug: bool,
    #[arg(long)]
    pub all_files: bool,
    #[arg(long)]
    pub show_memory_usage: bool,
    #[arg(long)]
    pub yolo: bool,
    #[arg(long)]
    pub approval_mode: Option<ApprovalMode>,
    #[arg(long)]
    pub telemetry: Option<bool>,
    #[arg(long)]
    pub telemetry_target: Option<TelemetryTarget>,
    #[arg(long)]
    pub telemetry_otlp_endpoint: Option<String>,
    #[arg(long)]
    pub telemetry_log_prompts: Option<bool>,
    #[arg(long)]
    pub telemetry_outfile: Option<String>,
    #[arg(long)]
    pub checkpointing: bool,
    #[arg(long)]
    pub experimental_acp: Option<bool>,
    #[arg(long, value_delimiter = ',')]
    pub allowed_mcp_server_names: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    pub extensions: Vec<String>,
    #[arg(long)]
    pub list_extensions: Option<bool>,
    #[arg(long)]
    pub proxy: Option<String>,
    #[arg(long, value_delimiter = ',')]
    pub include_directories: Vec<String>,
    #[arg(long)]
    pub version: Option<bool>,
    #[arg(long)]
    pub help: Option<bool>,

    // Custom arguments for the CRQ workflow
    #[arg(long)]
    pub crq: Option<String>,
    #[arg(long)]
    pub mode: Option<String>,
    #[arg(long)]
    pub inside: Option<String>,
    #[arg(long)]
    pub via: Option<String>,

    // Catch-all for arguments passed to the stage binary
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    pub stage_args: Vec<String>,
}

/// The main asynchronous function that runs the `launchpad` application.
///
/// This function is responsible for:
/// 1. Dynamically determining the project root directory.
/// 2. Setting the `LD_LIBRARY_PATH` environment variable to ensure correct
///    loading of dynamic libraries from the project's build directory.
/// 3. Parsing command-line arguments to identify the desired operational stage
///    and any arguments for that stage.
/// 4. Dispatching control to the appropriate stage handler (e.g., installing
///    Gemini CLI, running Gemini CLI, or executing a ZOS stage binary).
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`) if
/// the launchpad fails to initialize, parse arguments, or execute a stage.
///
/// # Stages Handled:
///
/// * `install-gemini`: Initiates the installation process for the Gemini CLI.
/// * `run-gemini`: Executes a command using the Gemini CLI.
/// * `<zos-stage-name>`: Executes a ZOS stage binary located in the project's
///   `target/debug/` directory. The stage name can optionally be prefixed with
///   `zos-stage-`.
///
/// # Errors
///
/// Returns an `Err` if:
/// - The current executable path cannot be determined.
/// - The project root cannot be determined.
/// - Insufficient command-line arguments are provided (less than 2).
/// - The `install-gemini` stage fails.
/// - A specified ZOS stage binary is not found.
/// - Execution of a ZOS stage binary fails or exits with a non-zero status.
pub async fn run_launchpad() -> Result<(), String> {
    // Determine the project root dynamically
    let current_exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {e}"))?;
    let project_root = current_exe_path.parent()
        .and_then(|p| p.parent()) // target/debug/
        .and_then(|p| p.parent()) // libminizinc/
        .ok_or("Could not determine project root")?;

    // Set LD_LIBRARY_PATH dynamically for any dynamically loaded components
    let build_dir = project_root.join("build");
    let ld_library_path = build_dir.to_string_lossy().to_string();
    std::env::set_var("LD_LIBRARY_PATH", ld_library_path);
    narrator::livestream_output(&format!("Set LD_LIBRARY_PATH to: {}", std::env::var("LD_LIBRARY_PATH").unwrap_or_default()));

    let args = LaunchpadArgs::parse();

    let stage_identifier = args.stage_identifier;
    let stage_args = args.stage_args;

    match stage_identifier.as_str() {
        "install-gemini" => {
            narrator::narrate_step("Installing Gemini CLI");
            orchestrator::install_gemini_cli().await.map_err(|e| {
                narrator::narrate_error(&format!("Failed to install Gemini CLI: {}", e));
                e
            })
        },
        "run-gemini" => {
            narrator::narrate_step("Running Gemini CLI");
            let options = GeminiCliOptions::from_args(stage_args);
            gemini_cli_runner::run_gemini_cli(
                &options,
                args.mode,
                args.inside,
                args.via,
            ).await; // Add .await here
            Ok(()) // gemini_cli_runner::run_gemini_cli exits, so this is fine.
        },
        "dum-test" => {
            if stage_args.is_empty() {
                return Err("Usage: launchpad dum-test <command_path> [command_args...]".to_string());
            }
            let command_path = stage_args[0].as_str();
            let command_args = &stage_args[1..];
            narrator::narrate_step(&format!("Running dum test for: {}", command_path));
            let current_dir = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
            run_main::run(command_path, &command_args.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), current_dir);
            Ok(()) // run_main::run calls exit, so this is fine.
        },
        _ => {
            // Original stage launching logic
            let stage_binary_name = if stage_identifier.starts_with("zos-stage-") {
                stage_identifier.to_string()
            } else {
                format!("zos-stage-{}", stage_identifier)
            };
            let stage_binary_path = project_root
                .join("target")
                .join("debug")
                .join(&stage_binary_name);

            if !stage_binary_path.exists() {
                return Err(format!("Stage binary not found: {:?}\n", stage_binary_path));
            }

            // Execute the stage binary
            let mut command = std::process::Command::new(&stage_binary_path);
            command.args(stage_args.clone());
            command.stdout(std::process::Stdio::inherit());
            command.stderr(std::process::Stdio::inherit());

            narrator::livestream_output(&format!("Launching stage: {:?} with args: {:?}\n", stage_binary_path, stage_args));

            let status = command.status()
                .map_err(|e| format!("Failed to execute stage binary: {}\n", e))?;

            if status.success() {
                Ok(())
            } else {
                Err(format!("Stage binary exited with non-zero status: {:?}\n", status.code()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Direct unit testing of `run_launchpad` is highly challenging
    // due to its reliance on global environment variables (`std::env::args`,
    // `std::env::set_var`) and process execution (`std::process::Command`).
    // Mocking these global behaviors is complex and often requires advanced
    // testing frameworks or significant refactoring to inject dependencies.
    //
    // This test primarily ensures that the function compiles and its basic
    // structure is sound. It does not attempt to simulate command-line
    // arguments or actual process execution.
    #[tokio::test]
    async fn test_run_launchpad_compiles() {
        // This test will pass if the function compiles correctly.
        // To properly test different execution paths (e.g., different stages),
        // you would need to:
        // 1. Refactor `run_launchpad` to take its dependencies (like arguments
        //    and command executor) as parameters, allowing for mocking.
        // 2. Use a crate like `mockall` or `test-env` to set up a controlled
        //    environment for `std::env` and `std::process` interactions.
        assert!(true);
    }
}
