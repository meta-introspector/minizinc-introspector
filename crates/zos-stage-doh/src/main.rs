use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use clap::Parser;

#[derive(Debug, Deserialize, Serialize)]
struct Workflow {
    name: String,
    on: serde_yaml::Value, // Keeping this as Value for now, can be refined if needed
    jobs: HashMap<String, Job>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Job {
    #[serde(rename = "runs-on")]
    runs_on: String,
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Step {
    name: String,
    #[serde(default)]
    uses: Option<String>,
    #[serde(default)]
    run: Option<String>,
    #[serde(default)]
    with: Option<HashMap<String, String>>,
    #[serde(default)]
    env: Option<HashMap<String, String>>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_version_flag = true, disable_help_flag = true)]
struct DohArgs {
    /// Path to the GitHub Actions workflow YAML file.
    workflow_file_path: String,

    #[arg(long)]
    crq: Option<String>,
    #[arg(long)]
    model: Option<String>,
    #[arg(long)]
    mode: Option<String>,
    #[arg(long)]
    via: Option<String>,
}

fn main() {
    println!("Hello from zos-stage-doh!");
    let args = DohArgs::parse();
    println!("Arguments received: {:?}
", args);

    let workflow_file_path = &args.workflow_file_path;
    println!("Attempting to parse workflow file: {}", workflow_file_path);

    let file_content = match fs::read_to_string(workflow_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading workflow file: {}", e);
            return;
        }
    };

    let workflow: Workflow = match serde_yaml::from_str(&file_content) {
        Ok(wf) => wf,
        Err(e) => {
            eprintln!("Error parsing YAML: {}", e);
            return;
        }
    };

    println!("Successfully parsed workflow: {:?}
", workflow);

    // Execute jobs
    for (job_name, job) in workflow.jobs {
        println!("\nExecuting job: {}", job_name);
        println!("  Runs on: {}", job.runs_on);

        // Execute steps
        for step in job.steps {
            println!("\n  Executing step: {}", step.name);

            // Handle environment variables for the step
            let mut step_env = HashMap::new();
            if let Some(env_vars) = step.env {
                for (key, value) in env_vars {
                    println!("    Setting env: {}={}", key, value);
                    step_env.insert(key, value);
                }
            }

            if let Some(uses) = step.uses {
                println!("    Uses: {}", uses);
                // Here, you would implement logic to handle 'uses' actions.
                // For now, we'll just print a message.
                println!("    (Action not implemented yet)");
            } else if let Some(mut run) = step.run {
                println!("    Run: {}", run);

                // Perform variable substitution
                if let Some(crq_val) = &args.crq {
                    run = run.replace("${{CRQ_ID}}", crq_val);
                }
                if let Some(model_val) = &args.model {
                    run = run.replace("${{GEMINI_MODEL}}", model_val);
                }

                // Execute the shell command
                let mut command = if cfg!(target_os = "windows") {
                    let mut cmd = Command::new("cmd");
                    cmd.arg("/C");
                    cmd
                } else {
                    let mut cmd = Command::new("sh");
                    cmd.arg("-c");
                    cmd
                };

                command.arg(&run);

                // Add step-specific environment variables
                for (key, value) in step_env {
                    command.env(key, value);
                }

                let output = command.output();

                match output {
                    Ok(output) => {
                        if output.status.success() {
                            println!("    Command succeeded.");
                            if !output.stdout.is_empty() {
                                println!("    Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
                            }
                        } else {
                            eprintln!("    Command failed with status: {:?}", output.status.code());
                            if !output.stdout.is_empty() {
                                eprintln!("    Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
                            }
                            if !output.stderr.is_empty() {
                                eprintln!("    Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("    Failed to execute command: {}", e);
                    }
                }
            }
        }
    }
}
