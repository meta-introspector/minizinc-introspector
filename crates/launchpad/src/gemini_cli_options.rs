
//! This module defines the data structures for representing the command-line
//! options of the Gemini CLI. These structures can be used to parse, validate,
//! and construct Gemini CLI commands programmatically.

use clap::ValueEnum;

/// Represents the possible values for the `--approval-mode` option.
#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum, serde::Serialize, serde::Deserialize)]
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

impl std::fmt::Display for ApprovalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents the possible values for the `--telemetry-target` option.
#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum, serde::Serialize, serde::Deserialize)]
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

use clap::Args;

/// Represents the command-line options for the Gemini CLI.
///
/// This struct provides a structured way to define and manage the various
/// options that can be passed to the `gemini` executable.
#[derive(Debug, Args)]
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

impl GeminiCliOptions {
    pub fn from_args(args: Vec<String>) -> Self {
        let cmd = clap::Command::new("gemini-cli").disable_help_flag(true)
            .arg(clap::Arg::new("model").long("model").action(clap::ArgAction::Set))
            .arg(clap::Arg::new("prompt").long("prompt").action(clap::ArgAction::Set))
            .arg(clap::Arg::new("prompt-interactive").long("prompt-interactive").action(clap::ArgAction::Set))
            .arg(clap::Arg::new("sandbox").long("sandbox").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("sandbox-image").long("sandbox-image").action(clap::ArgAction::Set))
            .arg(clap::Arg::new("debug").long("debug").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("all-files").long("all-files").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("show-memory-usage").long("show-memory-usage").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("yolo").long("yolo").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("approval-mode").long("approval-mode").value_parser(clap::value_parser!(ApprovalMode)))
            .arg(clap::Arg::new("telemetry").long("telemetry").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("telemetry-target").long("telemetry-target").value_parser(clap::value_parser!(TelemetryTarget)))
            .arg(clap::Arg::new("telemetry-otlp-endpoint").long("telemetry-otlp-endpoint").action(clap::ArgAction::Set))
            .arg(clap::Arg::new("telemetry-log-prompts").long("telemetry-log-prompts").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("telemetry-outfile").long("telemetry-outfile").action(clap::ArgAction::Set))
            .arg(clap::Arg::new("checkpointing").long("checkpointing").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("experimental-acp").long("experimental-acp").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("allowed-mcp-server-names").long("allowed-mcp-server-names").value_delimiter(',').action(clap::ArgAction::Append))
            .arg(clap::Arg::new("extensions").long("extensions").value_delimiter(',').action(clap::ArgAction::Append))
            .arg(clap::Arg::new("list-extensions").long("list-extensions").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("proxy").long("proxy").action(clap::ArgAction::Set))
            .arg(clap::Arg::new("include-directories").long("include-directories").value_delimiter(',').action(clap::ArgAction::Append))
            .arg(clap::Arg::new("version").long("version").action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new("help").long("help").action(clap::ArgAction::SetTrue));

        let matches = cmd.try_get_matches_from(args).unwrap_or_else(|e| e.exit());

        // Manually construct GeminiCliOptions from matches
        let mut options = GeminiCliOptions::default();

        options.model = matches.get_one::<String>("model").map(|s| s.to_string());
        options.prompt = matches.get_one::<String>("prompt").map(|s| s.to_string());
        options.prompt_interactive = matches.get_one::<String>("prompt-interactive").map(|s| s.to_string());
        options.sandbox = matches.get_one::<bool>("sandbox").copied();
        options.sandbox_image = matches.get_one::<String>("sandbox-image").map(|s| s.to_string());
        options.debug = matches.get_flag("debug");
        options.all_files = matches.get_flag("all-files");
        options.show_memory_usage = matches.get_flag("show-memory-usage");
        options.yolo = matches.get_flag("yolo");
        options.approval_mode = matches.get_one::<ApprovalMode>("approval-mode").copied();
        options.telemetry = matches.get_one::<bool>("telemetry").copied();
        options.telemetry_target = matches.get_one::<TelemetryTarget>("telemetry-target").copied();
        options.telemetry_otlp_endpoint = matches.get_one::<String>("telemetry-otlp-endpoint").map(|s| s.to_string());
        options.telemetry_log_prompts = matches.get_one::<bool>("telemetry-log-prompts").copied();
        options.telemetry_outfile = matches.get_one::<String>("telemetry-outfile").map(|s| s.to_string());
        options.checkpointing = matches.get_flag("checkpointing");
        options.experimental_acp = matches.get_one::<bool>("experimental-acp").copied();
        options.allowed_mcp_server_names = matches.get_many::<String>("allowed-mcp-server-names").map(|v| v.map(|s| s.to_string()).collect()).unwrap_or_default();
        options.extensions = matches.get_many::<String>("extensions").map(|v| v.map(|s| s.to_string()).collect()).unwrap_or_default();
        options.list_extensions = matches.get_one::<bool>("list-extensions").copied();
        options.proxy = matches.get_one::<String>("proxy").map(|s| s.to_string());
        options.include_directories = matches.get_many::<String>("include-directories").map(|v| v.map(|s| s.to_string()).collect()).unwrap_or_default();
        options.version = matches.get_one::<bool>("version").copied();
        options.help = matches.get_one::<bool>("help").copied();

        options
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
