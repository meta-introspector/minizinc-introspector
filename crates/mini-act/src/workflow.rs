use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub on: On,
    pub env: Option<HashMap<String, String>>,
    pub jobs: HashMap<String, Job>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)] // Allows for multiple possible structures for 'on'
pub enum On {
    #[serde(rename = "push")]
    Push(Option<Push>),
    #[serde(rename = "pull_request")]
    PullRequest(Option<PullRequest>),
    // Add other trigger types as needed
    String(String), // For simple 'on: push' or 'on: [push, pull_request]'
    List(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct Push {
    pub branches: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub branches: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    #[serde(rename = "runs-on")]
    pub runs_on: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub uses: Option<String>,
    pub run: Option<String>,
}