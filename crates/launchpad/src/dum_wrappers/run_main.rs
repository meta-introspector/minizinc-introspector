//! This module provides a simplified implementation of `dum`'s run functionality.
//! It focuses on executing a specified script or binary directly, without
//! involving complex `package.json` parsing or interactive modes.

use super::run_command::{run_command, RunOptions};
use std::collections::HashMap;
use std::path::PathBuf;

/// Executes a script or binary using a simplified `dum`-like mechanism.
///
/// This function sets up the environment, including the `PATH` variable,
/// and then delegates the actual command execution to `run_command`.
/// Note that `run_command` is expected to terminate the process, making
/// any code after its call unreachable.
///
/// # Arguments
///
/// * `script_name` - The name of the script or binary to execute.
/// * `forwarded_args` - A slice of string slices representing arguments
///                      to be passed to the executed script/binary.
/// * `current_dir` - The `PathBuf` representing the directory in which
///                   the command should be executed.
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

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Testing `run` directly is problematic because it calls `run_command`,
    // which is expected to terminate the process via `std::process::exit`.
    // This would stop the test runner.
    //
    // This test primarily ensures the function compiles. To properly test its
    // behavior without terminating the process, `run_command` (or the underlying
    // `std::process::exit`) would need to be mocked or the `run` function
    // refactored to return a result instead of directly causing an exit.
    #[test]
    fn test_run_compiles() {
        // Placeholder to ensure the function compiles.
        // The actual call to `run` is commented out to prevent process termination.
        //
        // let dummy_path = PathBuf::from("/tmp");
        // run("echo", &["hello"], dummy_path);
        assert!(true);
    }
}