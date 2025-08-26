use super::run_command::{run_command, RunOptions};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;

// This is a simplified version of dum's run function,
// focusing on executing a script or binary directly.
// It does not include package.json parsing or interactive modes.
pub fn run(script_name: &str, forwarded_args: &[&str], current_dir: PathBuf) {
    let envs = HashMap::from([("PATH".to_string(), std::env::var("PATH").unwrap_or_default())]);

    run_command(
        script_name,
        forwarded_args,
        &RunOptions {
            current_dir,
            envs,
        },
    );
    // run_command calls exit, so this part is unreachable
}
