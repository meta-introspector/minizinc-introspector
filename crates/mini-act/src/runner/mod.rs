pub mod job_runner;
pub mod step_executor;
pub mod env_processor;

use crate::workflow::{Workflow, On};
use std::collections::HashMap;

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
        job_runner::run_job(job, &Some(combined_env.clone())); // Use job_runner::run_job
    }
}