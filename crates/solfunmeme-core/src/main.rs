//! This is the main entry point for the `solfunmeme-core` executable.
//! It initializes the asynchronous runtime and delegates the primary application
//! logic to the `launchpad_app` module or other subcommands.

mod orchestrator;
mod narrator;
mod dum_wrappers;
mod gemini_cli_options;
mod stages;
mod tmux_controller_commands; // New: Module for tmux_controller commands
mod gemini_commands; // New: Module for gemini_commands

// The actual logic for launchpad is now in this module
mod launchpad_app;

use gemini_utils::gemini_eprintln;
use clap::{Parser, Subcommand};

/// Top-level CLI for the solfunmeme-core application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Enumeration of top-level commands.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs the launchpad application logic.
    Launchpad(launchpad_app::LaunchpadArgs),
    /// Provides commands for interacting with tmux sessions.
    Tmux(tmux_controller_commands::Cli), // New: Tmux subcommand
}

/// The asynchronous main function for the `solfunmeme-core` application.
///
/// This function serves as the binary's entry point. It sets up the Tokio
/// runtime and then calls `launchpad_app::run_launchpad` or dispatches
/// to other subcommands.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`) if
/// the executed command encounters an issue.
#[tokio::main]
async fn main() -> Result<(), String> {
    gemini_eprintln!("Starting solfunmeme-core...");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Launchpad(args) => {
            // Temporarily set environment variables for launchpad_app
            // This will be refactored later when zos-bootstrap is integrated
            let current_exe_path = std::env::current_exe()
                .map_err(|e| format!("Failed to get current executable path: {}", e.to_string()))?;
            let project_root = current_exe_path.parent()
                .and_then(|p| p.parent()) // target/debug/
                .and_then(|p| p.parent()) // libminizinc/
                .ok_or("Could not determine project root")?;

            let build_dir = project_root.join("build");
            let ld_library_path = build_dir.to_string_lossy().to_string();
            std::env::set_var("LD_LIBRARY_PATH", ld_library_path);
            narrator::livestream_output(&format!("Set LD_LIBRARY_PATH to: {}", std::env::var("LD_LIBRARY_PATH").unwrap_or_default()));

            launchpad_app::run_launchpad().await
        },
        Commands::Tmux(tmux_cli) => {
            // Dispatch to the tmux_controller_commands's main logic
            // This will need to be adapted to call the handle_*_command functions directly
            // For now, we'll just call a placeholder or the actual main if it exists
            tmux_controller_commands::handle_cli_command(tmux_cli).await
        },
    }
}

#[cfg(test)]
mod tests {
    // Note: The `main` function is typically not unit-tested directly as its
    // primary role is to serve as the application's entry point and delegate
    // to other functions. The core logic resides in `launchpad_app::run_launchpad`,
    // which is where testing efforts are focused.
    // This placeholder test ensures the `main` function compiles.
    #[tokio::test]
    async fn test_main_compiles() {
        assert!(true);
    }
}
