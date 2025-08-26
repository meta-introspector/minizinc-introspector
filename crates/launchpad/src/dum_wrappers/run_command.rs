use std::process::{Command, exit};
use std::path::PathBuf;
use std::collections::HashMap;

pub use super::run_options::RunOptions;

pub fn run_command(script: &str, args: &[&str], options: &RunOptions) {
    let mut command = {
        if cfg!(target_os = "windows") {
            let mut command = Command::new("cmd");
            command.arg("/C").arg(script);
            command
        } else {
            let mut command = Command::new("sh");
            command
                .arg("-c")
                .arg(format!("{} \"$@\"", script))
                .arg("sh");
            command
        }
    };

    let status = command
        .args(args)
        .envs(&options.envs)
        .current_dir(&options.current_dir)
        .status()
        .expect("failed to execute the command");

    exit(status.code().unwrap_or(1));
}