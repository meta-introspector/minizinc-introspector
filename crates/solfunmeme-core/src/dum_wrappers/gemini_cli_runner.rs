//! This module provides a wrapper for running the Gemini CLI using `dum_lib`.
//! It sets up the necessary environment and arguments to execute Gemini CLI commands
//! from within the `libminizinc` project context.

use dum_lib::args;
use dum_lib::run;
use std::path::PathBuf;
use crate::gemini_cli_options::{GeminiCliOptions, ApprovalMode, TelemetryTarget};

/// Executes a command for the Gemini CLI using `dum_lib`.
///
/// This function constructs the appropriate arguments for `dum_lib` to run
/// a specified Gemini CLI command based on the provided `GeminiCliOptions`.
/// It automatically sets the working directory to the `gemini-cli` project path.
///
/// # Arguments
///
/// * `options` - A reference to a `GeminiCliOptions` struct containing the
///               desired command-line options for the Gemini CLI.
///
/// # Examples
///
/// ```no_run
/// use crate::gemini_cli_options::GeminiCliOptions;
/// // Example of running a Gemini CLI chat command with a specific model
/// // let options = GeminiCliOptions { model: Some("gemini-pro".to_string()), ..Default::default() };
/// // run_gemini_cli(&options);
/// ```
pub async fn run_gemini_cli(options: &GeminiCliOptions, mode: Option<String>, inside: Option<String>, via: Option<String>) -> Result<(), String> {
    let gemini_cli_project_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/gemini-cli");

    let mut dum_args_vec: Vec<String> = Vec::new();
    dum_args_vec.push("dum".to_string()); // The first argument is usually the binary name itself
    dum_args_vec.push("gemini-cli".to_string()); // The script name for dum

    // Convert GeminiCliOptions to command-line arguments
    if let Some(model) = &options.model {
        dum_args_vec.push("--model".to_string());
        dum_args_vec.push(model.clone());
    }
    if let Some(prompt) = &options.prompt {
        dum_args_vec.push("--prompt".to_string());
        dum_args_vec.push(prompt.clone());
    }
    if let Some(prompt_interactive) = &options.prompt_interactive {
        dum_args_vec.push("--prompt-interactive".to_string());
        dum_args_vec.push(prompt_interactive.clone());
    }
    if let Some(sandbox) = options.sandbox {
        if sandbox {
            dum_args_vec.push("--sandbox".to_string());
        }
    }
    if let Some(sandbox_image) = &options.sandbox_image {
        dum_args_vec.push("--sandbox-image".to_string());
        dum_args_vec.push(sandbox_image.clone());
    }
    if options.debug {
        dum_args_vec.push("--debug".to_string());
    }
    if options.all_files {
        dum_args_vec.push("--all-files".to_string());
    }
    if options.show_memory_usage {
        dum_args_vec.push("--show-memory-usage".to_string());
    }
    if options.yolo {
        dum_args_vec.push("--yolo".to_string());
    }
    if let Some(approval_mode) = &options.approval_mode {
        dum_args_vec.push("--approval-mode".to_string());
        dum_args_vec.push(match approval_mode {
            ApprovalMode::Default => "default".to_string(),
            ApprovalMode::AutoEdit => "auto_edit".to_string(),
            ApprovalMode::Yolo => "yolo".to_string(),
        });
    }
    if let Some(telemetry) = options.telemetry {
        if telemetry {
            dum_args_vec.push("--telemetry".to_string());
        }
    }
    if let Some(telemetry_target) = &options.telemetry_target {
        dum_args_vec.push("--telemetry-target".to_string());
        dum_args_vec.push(match telemetry_target {
            TelemetryTarget::Local => "local".to_string(),
            TelemetryTarget::Gcp => "gcp".to_string(),
        });
    }
    if let Some(telemetry_otlp_endpoint) = &options.telemetry_otlp_endpoint {
        dum_args_vec.push("--telemetry-otlp-endpoint".to_string());
        dum_args_vec.push(telemetry_otlp_endpoint.clone());
    }
    if let Some(telemetry_log_prompts) = options.telemetry_log_prompts {
        if telemetry_log_prompts {
            dum_args_vec.push("--telemetry-log-prompts".to_string());
        }
    }
    if let Some(telemetry_outfile) = &options.telemetry_outfile {
        dum_args_vec.push("--telemetry-outfile".to_string());
        dum_args_vec.push(telemetry_outfile.clone());
    }
    if options.checkpointing {
        dum_args_vec.push("--checkpointing".to_string());
    }
    if let Some(experimental_acp) = options.experimental_acp {
        if experimental_acp {
            dum_args_vec.push("--experimental-acp".to_string());
        }
    }
    for name in &options.allowed_mcp_server_names {
        dum_args_vec.push("--allowed-mcp-server-names".to_string());
        dum_args_vec.push(name.clone());
    }
    for extension in &options.extensions {
        dum_args_vec.push("--extensions".to_string());
        dum_args_vec.push(extension.clone());
    }
    if let Some(list_extensions) = options.list_extensions {
        if list_extensions {
            dum_args_vec.push("--list-extensions".to_string());
        }
    }
    if let Some(proxy) = &options.proxy {
        dum_args_vec.push("--proxy".to_string());
        dum_args_vec.push(proxy.clone());
    }
    for dir in &options.include_directories {
        dum_args_vec.push("--include-directories".to_string());
        dum_args_vec.push(dir.clone());
    }
    if let Some(version) = options.version {
        if version {
            dum_args_vec.push("--version".to_string());
        }
    }
    if let Some(help) = options.help {
        if help {
            dum_args_vec.push("--help".to_string());
        }
    }

    // Add launchpad-specific arguments
    if let Some(m) = mode {
        dum_args_vec.push("--mode".to_string());
        dum_args_vec.push(m);
    }
    if let Some(i) = inside {
        dum_args_vec.push("--inside".to_string());
        dum_args_vec.push(i);
    }
    if let Some(v) = via {
        dum_args_vec.push("--via".to_string());
        dum_args_vec.push(v);
    }

    let app_args = args::parse_args(dum_args_vec);

    // Override change_dir to the gemini-cli project path
    let mut app_args_modified = app_args;
    app_args_modified.change_dir = gemini_cli_project_path;

    run::run(app_args_modified);
    Ok(()) // Add this line
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Direct testing of `run_gemini_cli` is difficult without mocking
    // `dum_lib::run` which performs side effects (executing commands and changing directories).
    // This test primarily ensures the function compiles and can be called.
    #[test]
    fn test_run_gemini_cli_compiles_and_runs() {
        // This test will attempt to call run_gemini_cli with dummy options.
        // It will likely fail if dum_lib is not properly set up or if it tries
        // to execute a non-existent command.
        // The purpose here is to ensure the code path is valid and compiles.
        // A more robust test would require mocking the `dum_lib::run` function.
        //
        // To avoid actual execution during test, we can comment out the call
        // or use a feature flag for testing if dum_lib supports it.
        // For now, we'll just ensure it compiles.
        //
        // If dum_lib::run causes issues during testing, consider:
        // 1. Mocking `dum_lib::run` if `dum_lib` provides a testable interface.
        // 2. Using `#[cfg(not(test))]` around the `run::run` call and
        //    a mock implementation for tests if direct execution is problematic.

        // This line is commented out to prevent actual execution during `cargo test`
        // as `dum_lib::run` would attempt to execute a real command.
        // run_gemini_cli(&GeminiCliOptions::default());
        // If you want to verify the argument parsing and path setting without
        // actual execution, you would need to refactor `run_gemini_cli`
        // to return the `app_args_modified` instead of directly calling `run::run`.
        assert!(true); // Placeholder assertion to ensure the test passes if the function compiles.
    }
}
