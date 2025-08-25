mod workflow;
mod runner;

use std::fs;

fn main() {
    let workflow_path = ".github/workflows/rust-check.yml";
    let workflow_content = fs::read_to_string(workflow_path).expect("Failed to read workflow file");

    let workflow: workflow::Workflow = serde_yaml::from_str(&workflow_content).expect("Failed to parse workflow file");

    runner::run_workflow(&workflow);
}