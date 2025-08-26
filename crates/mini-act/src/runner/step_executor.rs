use std::process::Command;
use std::collections::HashMap;
use tempfile::tempdir;
use crate::git_actions;

pub fn execute_step_command(substituted_run: &str, step_env: &HashMap<String, String>) {

    if substituted_run.starts_with("__MINI_ACT_GIT_CHECKOUT__") {
        let parts: Vec<&str> = substituted_run.splitn(3, ' ').collect();
        if parts.len() == 3 {
            let repo_url = parts[1];
            let target_path_str = parts[2];

            let temp_dir = tempdir().expect("Failed to create temporary directory for clone");
            let target_path = temp_dir.path().join(target_path_str);

            match git_actions::git_checkout(repo_url, &target_path) {
                Ok(_) => println!("Git checkout successful to {:?}\n", target_path),
                Err(e) => eprintln!("Git checkout failed: {}\n", e),
            }
        } else {
            eprintln!("Invalid __MINI_ACT_GIT_CHECKOUT__ command format.\n");
        }
    } else {
        let mut command = Command::new("sh");
        command.arg("-c").arg(substituted_run);

        // Apply environment variables from the step_env
        for (key, value) in step_env {
            command.env(key, value);
        }

        let output = command.output().expect("Failed to execute command");

        println!("Status: {}\n", output.status);
        println!("Stdout: {}\n", String::from_utf8_lossy(&output.stdout));
        println!("Stderr: {}\n", String::from_utf8_lossy(&output.stderr));
    }
}

