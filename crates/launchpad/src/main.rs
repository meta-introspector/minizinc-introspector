use std::env;
use std::process::{Command, Stdio};


fn main() -> Result<(), String> {
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
    eprintln!("Set LD_LIBRARY_PATH to: {}", env::var("LD_LIBRARY_PATH").unwrap_or_default());

    // The first argument after the binary name is expected to be the stage identifier
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: launchpad <stage_identifier> [stage_args...]".to_string());
    }

    let stage_identifier = &args[1];
    let stage_args = &args[2..]; // Arguments to pass to the stage binary

    // Construct the path to the stage binary
    // For now, assume stage binaries are in target/debug/
    // In a real scenario, this would be more sophisticated (e.g., based on a manifest)
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
        return Err(format!("Stage binary not found: {:?}", stage_binary_path));
    }

    // Execute the stage binary
    let mut command = Command::new(&stage_binary_path);
    command.args(stage_args);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    eprintln!("Launching stage: {:?} with args: {:?}", stage_binary_path, stage_args);

    let status = command.status()
        .map_err(|e| format!("Failed to execute stage binary: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Stage binary exited with non-zero status: {:?}", status.code()))
    }
}
