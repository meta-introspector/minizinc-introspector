
//! This module defines the data structures for representing the command-line
//! options of the Gemini CLI. These structures can be used to parse, validate,
//! and construct Gemini CLI commands programmatically.

/// Represents the possible values for the `--approval-mode` option.
#[derive(Debug, PartialEq, Eq)]
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
            "yolo" => ApprovalMode::Yolo,
            _ => ApprovalMode::Default, // Fallback for unknown values
        }
    }
}

/// Represents the possible values for the `--telemetry-target` option.
#[derive(Debug, PartialEq, Eq)]
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

/// Represents the command-line options for the Gemini CLI.
///
/// This struct provides a structured way to define and manage the various
/// options that can be passed to the `gemini` executable.
#[derive(Debug)]
pub struct GeminiCliOptions {
    pub model: Option<String>,
    pub prompt: Option<String>,
    pub prompt_interactive: Option<String>,
    pub sandbox: Option<bool>,
    pub sandbox_image: Option<String>,
    pub debug: bool,
    pub all_files: bool,
    pub show_memory_usage: bool,
    pub yolo: bool,
    pub approval_mode: Option<ApprovalMode>,
    pub telemetry: Option<bool>,
    pub telemetry_target: Option<TelemetryTarget>,
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
    pub help: Option<bool>,
}

impl Default for GeminiCliOptions {
    fn default() -> Self {
        GeminiCliOptions {
            model: None,
            prompt: None,
            prompt_interactive: None,
            sandbox: None,
            sandbox_image: None,
            debug: false,
            all_files: false,
            show_memory_usage: false,
            yolo: false,
            approval_mode: None,
            telemetry: None,
            telemetry_target: None,
            telemetry_otlp_endpoint: None,
            telemetry_log_prompts: None,
            telemetry_outfile: None,
            checkpointing: false,
            experimental_acp: None,
            allowed_mcp_server_names: Vec::new(),
            extensions: Vec::new(),
            list_extensions: None,
            proxy: None,
            include_directories: Vec::new(),
            version: None,
            help: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_cli_options_default() {
        let options = GeminiCliOptions::default();
        assert_eq!(options.debug, false);
        assert_eq!(options.all_files, false);
        assert_eq!(options.show_memory_usage, false);
        assert_eq!(options.yolo, false);
        assert!(options.model.is_none());
        assert!(options.extensions.is_empty());
    }

    #[test]
    fn test_approval_mode_from_str() {
        assert_eq!(ApprovalMode::from("default"), ApprovalMode::Default);
        assert_eq!(ApprovalMode::from("auto_edit"), ApprovalMode::AutoEdit);
        assert_eq!(ApprovalMode::from("yolo"), ApprovalMode::Yolo);
        assert_eq!(ApprovalMode::from("unknown"), ApprovalMode::Default);
    }

    #[test]
    fn test_telemetry_target_from_str() {
        assert_eq!(TelemetryTarget::from("local"), TelemetryTarget::Local);
        assert_eq!(TelemetryTarget::from("gcp"), TelemetryTarget::Gcp);
        assert_eq!(TelemetryTarget::from("unknown"), TelemetryTarget::Local);
    }
}
