use serde::{Serialize, Deserialize};
//use std::path::PathBuf;

// Re-introduce all fields from LaunchpadArgs
#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchArgs {
    pub stage_identifier: String,

    // Arguments for Gemini CLI
    pub model: Option<String>,
    pub prompt: Option<String>,
    pub prompt_interactive: Option<String>,
    pub sandbox: Option<bool>,
    pub sandbox_image: Option<String>,
    pub debug: bool,
    pub all_files: bool,
    pub show_memory_usage: bool,
    pub yolo: bool,
    pub approval_mode: Option<String>, // Changed to String for simplicity, will convert later
    pub telemetry: Option<bool>,
    pub telemetry_target: Option<String>, // Changed to String for simplicity, will convert later
    pub telemetry_otlp_endpoint: Option<String>,
    pub telemetry_log_prompts: Option<bool>,
    pub telemetry_outfile: Option<String>,
    pub checkpointing: bool,
    pub experimental_acp: Option<bool>,
    pub allowed_mcp_server_names: Vec<String>,
    pub extensions: Vec<String>,
    pub list_extensions: Option<bool>,
    pub proxy: Option<String>,
    pub include_directories: Vec<String>,
    pub version: Option<bool>,


    // Custom arguments for the CRQ workflow
    pub crq: Option<String>,
    pub mode: Option<String>,
    pub inside: Option<String>,
    pub via: Option<String>,
    pub crq_path: Option<String>, // New: Path to the CRQ file
    pub target_repo_url: Option<String>, // New: URL of the target repository
    pub workflow_file_in_repo: Option<String>, // New: Path to the workflow file within the target repository
    pub gemini_cli_path: Option<String>, // New: Path to the Gemini CLI executable
    pub gemini_instances: usize, // Number of Gemini instances to launch
    pub record_session: bool, // Whether to record the session with asciinema
    pub background_detached: bool, // Whether to launch Gemini in a detached background process

    // Catch-all for arguments passed to the stage binary
    pub stage_args: Vec<String>,
}
