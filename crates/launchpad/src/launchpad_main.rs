use std::env;
use std::process::{Command, Stdio};
use crate::orchestrator;
use crate::narrator;
use crate::dum_wrappers::run_main as dum_run_main;

pub async fn run_launchpad() -> Result<(), String> {
    // Determine the project root dynamically
    let current_exe_path = env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {e}"))?;
    let project_root = current_exe_path.parent()
        .and_then(|p| p.parent()) // target/debug/
        .and_then(|p| p.parent()) // libminizinc/
        .ok_or("Could not determine project root")?;

    // Set LD_LIBRARY_PATH dynamically for any dynamically loaded components
    let build_dir = project_root.join("build");
    let ld_library_path = build_dir.to_string_lossy().to_string();
    env::set_var("LD_LIBRARY_PATH", ld_library_path);
    narrator::livestream_output(&format!("Set LD_LIBRARY_PATH to: {}", env::var("LD_LIBRARY_PATH").unwrap_or_default()));

    // The first argument after the binary name is expected to be the stage identifier
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: launchpad <stage_identifier> [stage_args...]".to_string());
    }

    let stage_identifier = args.remove(1);
    let stage_args = args.split_off(1);

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
            orchestrator::run_gemini_cli(&stage_args.iter().map(|s| s.as_str()).collect::<Vec<&str>>()).await.map_err(|e| {
                narrator::narrate_error(&format!("Failed to run Gemini CLI: {}", e));
                e
            })
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
            let mut command = Command::new(&stage_binary_path);
            command.args(stage_args);
            command.stdout(Stdio::inherit());
            command.stderr(Stdio::inherit());

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
