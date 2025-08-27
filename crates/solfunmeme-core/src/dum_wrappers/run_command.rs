//! This module provides functionality to execute shell commands.
//! It abstracts away platform-specific command execution details and allows
//! specifying arguments, environment variables, and the working directory.

use std::process::{Command, exit};

pub use super::run_options::RunOptions;

/// Executes a given script or command in a new process.
///
/// This function constructs and executes a shell command, handling differences
/// between Windows and Unix-like operating systems. It sets the command's
/// arguments, environment variables, and current working directory based on
/// the provided `RunOptions`.
///
/// **Important:** This function calls `std::process::exit` with the exit code
/// of the executed command. Therefore, any code following a call to `run_command`
/// will not be executed.
///
/// # Arguments
///
/// * `script` - The name of the script or command to execute.
/// * `args` - A slice of string slices representing arguments to pass to the command.
/// * `options` - A reference to `RunOptions` containing the current directory
///               and environment variables for the command.
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

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Directly unit testing `run_command` is not feasible because it
    // executes external processes and calls `std::process::exit`, which
    // terminates the test runner. This makes it impossible to assert on its
    // behavior or return values.
    //
    // This test primarily ensures the function compiles.
    // Proper testing of
    // this function would require integration tests that run the actual
    // binary and check its side effects (e.g., file system changes, output).
    #[test]
    fn test_run_command_compiles() {
        // Placeholder to ensure the function compiles.
        // The actual call to `run_command` is commented out to prevent process termination.
        //
        // let dummy_options = RunOptions {
        //     current_dir: PathBuf::from("/tmp"),
        //     envs: HashMap::new(),
        // };
        // run_command("echo", &["hello"], &dummy_options);
        assert!(true);
    }
}
