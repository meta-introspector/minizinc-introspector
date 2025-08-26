use crate::workflow::{Workflow, Job, On};
use std::process::Command;
use std::collections::HashMap;

use tempfile::tempdir;
use crate::git_actions;

pub fn run_workflow(workflow: &Workflow) {
    let mut combined_env: HashMap<String, String> = HashMap::new();

    // Add workflow-level environment variables
    if let Some(workflow_env) = &workflow.env {
        combined_env.extend(workflow_env.clone());
    }

    // Add workflow_dispatch inputs as environment variables
    if let On::WorkflowDispatch(Some(dispatch)) = &workflow.on {
        if let Some(inputs) = &dispatch.inputs {
            for (input_name, input_props) in inputs {
                // For simplicity, we'll use the default value if available, otherwise an empty string
                // In a real scenario, these would come from the command line arguments
                let input_value = input_props.default.clone().unwrap_or_default();
                combined_env.insert(format!("INPUT_{}", input_name.to_uppercase()), input_value);
            }
        }
    }

    for (job_name, job) in &workflow.jobs {
        println!("Running job: {}", job_name);
        run_job(job, &Some(combined_env.clone())); // Pass the combined environment
    }
}

fn run_job(job: &Job, env: &Option<HashMap<String, String>>) {
    for step in &job.steps {
        if let Some(run) = &step.run {
            println!("Running step: {:?}
", step.name.as_deref().unwrap_or("")); // Added newline for clarity
            let substituted_run = substitute_env_vars(run, env);

            if substituted_run.starts_with("__MINI_ACT_GIT_CHECKOUT__") {
                let parts: Vec<&str> = substituted_run.splitn(3, ' ').collect();
                if parts.len() == 3 {
                    let repo_url = parts[1];
                    let target_path_str = parts[2];

                    // Create a temporary directory for the clone
                    let temp_dir = tempdir().expect("Failed to create temporary directory for clone");
                    let target_path = temp_dir.path().join(target_path_str);

                    match git_actions::git_checkout(repo_url, &target_path) {
                        Ok(_) => println!("Git checkout successful to {:?}
", target_path), // Added newline
                        Err(e) => eprintln!("Git checkout failed: {}
", e), // Added newline
                    }
                } else {
                    eprintln!("Invalid __MINI_ACT_GIT_CHECKOUT__ command format.
"); // Added newline
                }
            } else {
                let mut command = Command::new("sh");
                command.arg("-c").arg(substituted_run);

                // Apply environment variables from the combined_env
                if let Some(env_map) = env {
                    for (key, value) in env_map {
                        // No need to substitute here, as env_map already contains substituted values
                        command.env(key, value);
                    }
                }

                let output = command.output().expect("Failed to execute command");

                println!("Status: {}
", output.status);
                println!("Stdout: {}
", String::from_utf8_lossy(&output.stdout));
                println!("Stderr: {}
", String::from_utf8_lossy(&output.stderr));
            }
        }
    }
}

fn substitute_env_vars(run: &str, env: &Option<HashMap<String, String>>) -> String {
    let mut substituted_run = run.to_string();
    if let Some(env_map) = env {
        let mut changed = true;
        while changed {
            changed = false;
            for (key, value) in env_map {
                let placeholder = format!("${{{{ env.{} }}}}", key);
                if substituted_run.contains(&placeholder) {
                    substituted_run = substituted_run.replace(&placeholder, value);
                    changed = true;
                }
            }
        }
    }
    substituted_run
}
