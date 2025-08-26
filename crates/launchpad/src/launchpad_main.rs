use crate::orchestrator;
use crate::narrator;
use crate::dum_wrappers::gemini_cli_runner;

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

    // The first argument after the binary name is expected to be the stage identifier
    let mut args: Vec<String> = std::env::args().collect();
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
            gemini_cli_runner::run_gemini_cli(&stage_args.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
            Ok(()) // gemini_cli_runner::run_gemini_cli exits, so this is fine.
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
