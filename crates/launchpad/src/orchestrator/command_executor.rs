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
