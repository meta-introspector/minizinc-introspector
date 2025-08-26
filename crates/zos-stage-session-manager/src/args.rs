use clap::{Parser, ValueEnum};
use serde::{Serialize, Deserialize};

/// Represents the possible values for the `--approval-mode` option.
#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum ApprovalMode {
    Default,
    AutoEdit,
    Yolo,
}

impl From<&str> for ApprovalMode {
    fn from(s: &str) -> Self {
        match s {
            "default" => ApprovalMode::Default,
            "auto_edit" => ApprovalMode::AutoEdit,
            "yolo" => "yolo".to_string().into(), // This needs to be fixed, it should be ApprovalMode::Yolo
            _ => ApprovalMode::Default, // Fallback for unknown values
        }
    }
}

impl std::fmt::Display for ApprovalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents the possible values for the `--telemetry-target` option.
#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum TelemetryTarget {
    Local,
    Gcp,
}

impl From<&str> for TelemetryTarget {
    fn from(s: &str) -> Self {
        match s {
            "local" => TelemetryTarget::Local,
            "gcp" => TelemetryTarget::Gcp,
            _ => TelemetryTarget::Local, // Fallback for unknown values
        }
    }
}

impl std::fmt::Display for TelemetryTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct LaunchArgs {
    /// Number of Gemini instances to launch
    #[arg(short, long, default_value_t = 1)]
    pub gemini_instances: u8,

    /// Whether to record the session with asciinema
    #[arg(short, long, default_value_t = false)]
    pub record_session: bool,

    /// Launch a single Gemini instance in the background, detached from the current terminal
    #[arg(long, default_value_t = false)]
    pub background_detached: bool,

    // Arguments for Gemini CLI (mirrored from LaunchpadArgs)
    #[arg(long)]
    pub model: Option<String>,
    #[arg(long)]
    pub prompt: Option<String>,
    #[arg(long)]
    pub prompt_interactive: Option<String>,
    #[arg(long)]
    pub sandbox: Option<bool>,
    #[arg(long)]
    pub sandbox_image: Option<String>,
    #[arg(long)]
    pub debug: bool,
    #[arg(long)]
    pub all_files: bool,
    #[arg(long)]
    pub show_memory_usage: bool,
    #[arg(long)]
    pub yolo: bool,
    #[arg(long)]
    pub approval_mode: Option<String>, // Changed to String for simplicity, will convert later
    #[arg(long)]
    pub telemetry: Option<bool>,
    #[arg(long)]
    pub telemetry_target: Option<String>, // Changed to String for simplicity, will convert later
    #[arg(long)]
    pub telemetry_otlp_endpoint: Option<String>,
    #[arg(long)]
    pub telemetry_log_prompts: Option<bool>,
    #[arg(long)]
    pub telemetry_outfile: Option<String>,
    #[arg(long)]
    pub checkpointing: bool,
    #[arg(long)]
    pub experimental_acp: Option<bool>,
    #[arg(long, value_delimiter = ',')]
    pub allowed_mcp_server_names: Vec<String>,
    #[arg(long)]
    pub extensions: Vec<String>,
    #[arg(long)]
    pub list_extensions: Option<bool>,
    #[arg(long)]
    pub proxy: Option<String>,
    #[arg(long, value_delimiter = ',')]
    pub include_directories: Vec<String>,
    #[arg(long)]
    pub version: Option<bool>,
    #[arg(long)]
    pub help: Option<bool>,

    // Custom arguments for the CRQ workflow (mirrored from LaunchpadArgs)
    #[arg(long)]
    pub crq: Option<String>,
    #[arg(long)]
    pub mode: Option<String>,
    #[arg(long)]
    pub inside: Option<String>,
    #[arg(long)]
    pub via: Option<String>,
    #[arg(long)]
    pub crq_path: Option<String>,
    #[arg(long)]
    pub target_repo_url: Option<String>,
    #[arg(long)]
    pub workflow_file_in_repo: Option<String>,
}
