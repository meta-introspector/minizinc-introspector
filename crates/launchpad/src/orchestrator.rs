use std::process::{Command, Stdio};
use std::path::Path;

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

pub async fn install_gemini_cli() -> Result<(), String> {
    eprintln!("Installing Gemini CLI...");
    run_command("npm", &["install", "-g", "@google/gemini-cli"], None).await
}

pub async fn run_gemini_cli(args: &[&str]) -> Result<(), String> {
    eprintln!("Running Gemini CLI...");
    run_command("gemini", args, None).await
}

// Placeholder for dum integration
pub async fn run_dum_task(task_name: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    eprintln!("Running dum task: {}", task_name);
    let mut dum_args = vec![task_name];
    dum_args.extend_from_slice(args);
    run_command("dum", &dum_args, cwd).await
}

// Placeholder for cargo commands
pub async fn run_cargo_command(args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    eprintln!("Running cargo command: {:?}", args);
    run_command("cargo", args, cwd).await
}

// Placeholder for tmux commands
pub async fn run_tmux_command(args: &[&str]) -> Result<(), String> {
    eprintln!("Running tmux command: {:?}", args);
    run_command("tmux", args, None).await
}

// Placeholder for pchroot/act-run integration
pub async fn run_sandboxed_command(cmd: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    eprintln!("Running sandboxed command: {} {:?}", cmd, args);
    // This would involve setting up pchroot/act-run environment
    run_command(cmd, args, cwd).await
}
