//! This module defines the `RunOptions` struct, which encapsulates configuration
//! for executing external commands, including environment variables and the
//! working directory.

use std::collections::HashMap;
use std::path::PathBuf;

/// Represents the options for running an external command.
///
/// This struct holds the environment variables and the current working directory
/// that should be used when executing a command.
#[derive(Debug)]
pub struct RunOptions {
    /// A `HashMap` of environment variables to set for the command.
    pub envs: HashMap<String, String>,
    /// The `PathBuf` representing the current working directory for the command.
    pub current_dir: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_options_creation() {
        let mut envs = HashMap::new();
        envs.insert("KEY".to_string(), "VALUE".to_string());
        let current_dir = PathBuf::from("/test/dir");

        let options = RunOptions {
            envs: envs.clone(),
            current_dir: current_dir.clone(),
        };

        assert_eq!(options.envs.get("KEY"), Some(&"VALUE".to_string()));
        assert_eq!(options.current_dir, current_dir);
    }
}