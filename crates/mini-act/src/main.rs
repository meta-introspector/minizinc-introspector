mod workflow;
mod runner;
mod git_actions;

use std::fs;
use std::path::Path;

fn main() {
    let workflows_dir = Path::new(".github/workflows");

    if !workflows_dir.exists() || !workflows_dir.is_dir() {
        eprintln!("Error: .github/workflows directory not found or is not a directory.");
        return;
    }

    println!("Running all workflows in {:?}...", workflows_dir);

    for entry in fs::read_dir(workflows_dir).expect("Failed to read workflows directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "yml" || extension == "yaml" {
                    println!("\n--- Running workflow: {:?} ---", path);
                    let workflow_content = match fs::read_to_string(&path) {
                        Ok(content) => content,
                        Err(e) => {
                            eprintln!("Error reading workflow file {:?}: {}", path, e);
                            continue;
                        }
                    };

                    let workflow: workflow::Workflow = match serde_yaml::from_str(&workflow_content) {
                        Ok(wf) => wf,
                        Err(e) => {
                            eprintln!("Error parsing workflow file {:?}: {}", path, e);
                            continue;
                        }
                    };

                    runner::run_workflow(&workflow);
                }
            }
        }
    }
}