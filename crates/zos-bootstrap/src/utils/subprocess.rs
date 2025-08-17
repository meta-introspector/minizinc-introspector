use std::process::{Command, Output};
use crate::{Result, ZosError};

pub fn run_command(cmd: &str, args: &[&str]) -> Result<Output> {
    let output = Command::new(cmd)
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(ZosError::CommandFailed {
            command: format!("{} {}", cmd, args.join(" ")),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

pub fn run_command_with_env(cmd: &str, args: &[&str], env_vars: &[(&str, &str)]) -> Result<Output> {
    let mut command = Command::new(cmd);
    command.args(args);
    for (key, value) in env_vars {
        command.env(key, value);
    }
    let output = command.output()?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(ZosError::CommandFailed {
            command: format!("{} {}", cmd, args.join(" ")),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
