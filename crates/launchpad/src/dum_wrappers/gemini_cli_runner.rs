use super::run_command::run_command;
use super::run_options::RunOptions;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn run_gemini_cli(args: &[&str]) {
    let gemini_cli_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/gemini-cli/target/release/gemini-cli");

    let envs = HashMap::from([("PATH".to_string(), std::env::var("PATH").unwrap_or_default())]);

    // The current_dir for gemini-cli should probably be its own directory
    let current_dir = gemini_cli_path.parent().unwrap_or(&gemini_cli_path).to_path_buf();

    run_command(
        gemini_cli_path.to_str().unwrap(),
        args,
        &RunOptions {
            current_dir,
            envs,
        },
    );
}