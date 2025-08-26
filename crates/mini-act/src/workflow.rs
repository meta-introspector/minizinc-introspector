use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub on: On,
    pub env: Option<HashMap<String, String>>,
    pub jobs: HashMap<String, Job>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)] // Allows for multiple possible structures for 'on'
pub enum On {
    #[serde(rename = "push")]
    Push(Option<Push>),
    #[serde(rename = "pull_request")]
    PullRequest(Option<PullRequest>),
    #[serde(rename = "workflow_dispatch")]
    WorkflowDispatch(Option<WorkflowDispatch>),
    // Add other trigger types as needed
    String(String),
    List(Vec<String>),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Push {
    pub branches: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub branches: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WorkflowDispatch {
    pub inputs: Option<HashMap<String, WorkflowDispatchInput>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WorkflowDispatchInput {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub default: Option<String>,
    #[serde(rename = "type")]
    pub input_type: Option<String>,
}


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Job {
    #[serde(rename = "runs-on")]
    pub runs_on: String,
    pub steps: Vec<Step>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub uses: Option<String>,
    pub run: Option<String>,
    pub env: Option<HashMap<String, String>>,
}