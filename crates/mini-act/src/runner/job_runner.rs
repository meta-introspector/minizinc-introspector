use crate::workflow::{Job};
use std::collections::HashMap;
use crate::runner::step_executor;
use crate::runner::env_processor;

pub fn run_job(job: &Job, inherited_env: &Option<HashMap<String, String>>) {
    for step in &job.steps {
        if let Some(run) = &step.run {
            println!("Running step: {:?}\n", step.name.as_deref().unwrap_or(""));
            
            let mut step_env: HashMap<String, String> = HashMap::new();

            // Inherit environment from job/workflow level
            if let Some(env_map) = inherited_env {
                step_env.extend(env_map.clone());
            }

            // Add job-level environment variables (if Job struct had an env field)
            // For now, assuming job.env is not yet implemented in workflow.rs

            // Add step-level environment variables, overriding inherited ones
            if let Some(env_map) = &step.env {
                step_env.extend(env_map.clone());
            }

            let substituted_run = env_processor::substitute_env_vars(run.as_str().unwrap_or_default(), &Some(step_env.clone()));

            step_executor::execute_step_command(&run, &step_env);
        }
    }
}

