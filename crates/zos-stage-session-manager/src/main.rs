use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Launches a new session
    Launch(LaunchArgs),
    /// Exports the current session configuration to a file
    Export(ExportArgs),
}

#[derive(Parser, Debug, Serialize, Deserialize)]
struct LaunchArgs {
    /// Number of Gemini instances to launch
    #[arg(short, long, default_value_t = 1)]
    gemini_instances: u8,

    /// Whether to record the session with asciinema
    #[arg(short, long, default_value_t = false)]
    record_session: bool,

    /// Launch a single Gemini instance in the background, detached from the current terminal
    #[arg(long, default_value_t = false)]
    background_detached: bool,

    // Arguments for Gemini CLI (mirrored from LaunchpadArgs)
    #[arg(long)]
    model: Option<String>,
    #[arg(long)]
    prompt: Option<String>,
    #[arg(long)]
    prompt_interactive: Option<String>,
    #[arg(long)]
    sandbox: Option<bool>,
    #[arg(long)]
    sandbox_image: Option<String>,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    all_files: bool,
    #[arg(long)]
    show_memory_usage: bool,
    #[arg(long)]
    yolo: bool,
    #[arg(long)]
    approval_mode: Option<String>, // Changed to String for simplicity, will convert later
    #[arg(long)]
    telemetry: Option<bool>,
    #[arg(long)]
    telemetry_target: Option<String>, // Changed to String for simplicity, will convert later
    #[arg(long)]
    telemetry_otlp_endpoint: Option<String>,
    #[arg(long)]
    telemetry_log_prompts: Option<bool>,
    #[arg(long)]
    telemetry_outfile: Option<String>,
    #[arg(long)]
    checkpointing: bool,
    #[arg(long)]
    experimental_acp: Option<bool>,
    #[arg(long, value_delimiter = ',')]
    allowed_mcp_server_names: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    extensions: Vec<String>,
    #[arg(long)]
    list_extensions: Option<bool>,
    #[arg(long)]
    proxy: Option<String>,
    #[arg(long, value_delimiter = ',')]
    include_directories: Vec<String>,
    #[arg(long)]
    version: Option<bool>,
    #[arg(long)]
    help: Option<bool>,

    // Custom arguments for the CRQ workflow (mirrored from LaunchpadArgs)
    #[arg(long)]
    crq: Option<String>,
    #[arg(long)]
    mode: Option<String>,
    #[arg(long)]
    inside: Option<String>,
    #[arg(long)]
    via: Option<String>,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
struct ExportArgs {
    /// Number of Gemini instances to launch
    #[arg(short, long, default_value_t = 1)]
    gemini_instances: u8,

    /// Whether to record the session with asciinema
    #[arg(short, long, default_value_t = false)]
    record_session: bool,

    /// Launch a single Gemini instance in the background, detached from the current terminal
    #[arg(long, default_value_t = false)]
    background_detached: bool,

    /// Output file path for the session configuration
    #[arg(short, long, default_value = "sessions/last_session.json")]
    output: String,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Launch(args) => {
            eprintln!("Starting session manager stage...");

            // Determine the project root dynamically to construct the path for asciicast
            let current_exe_path = env::current_exe()
                .map_err(|e| format!("Failed to get current executable path: {e}"))?;
            let project_root = current_exe_path.parent()
                .and_then(|p| p.parent()) // target/debug/
                .and_then(|p| p.parent()) // libminizinc/
                .ok_or("Could not determine project root")?;

            let asciicast_path = project_root.join("docs").join("asciicast_session.cast");

            // Handle background_detached launch
            if args.background_detached {
                if args.gemini_instances != 1 || args.record_session {
                    return Err("Cannot use --background-detached with multiple instances or session recording.".to_string());
                }
                eprintln!("Launching single Gemini instance in background, detached...");
                let shell_command = "nohup gemini --model gemini-2.5-flash --checkpointing=true > /dev/null 2>&1 &";
                eprintln!("Executing: {}", shell_command);

                let status = Command::new("bash")
                    .arg("-c")
                    .arg(shell_command)
                    .status()
                    .map_err(|e| format!("Failed to launch detached Gemini instance: {e}"))?;

                if !status.success() {
                    return Err(format!("Detached Gemini instance launch failed with status: {:?}", status.code()));
                }
                eprintln!("Detached Gemini instance launched successfully.");
                return Ok(());
            }

            // If recording is enabled, launch asciinema with the first gemini instance
            if args.record_session {
                eprintln!("Recording session to: {:?}", asciicast_path);
                let shell_command = format!(
                    "asciinema rec {} --command 'gemini --model gemini-2.5-flash --checkpointing=true'",
                    asciicast_path.to_string_lossy()
                );
                eprintln!("Executing: {}", shell_command);

                let status = Command::new("bash")
                    .arg("-c")
                    .arg(&shell_command)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()
                    .map_err(|e| format!("Failed to execute asciinema with gemini: {e}"))?;

                if !status.success() {
                    return Err(format!("Asciinema/Gemini exited with non-zero status: {:?}", status.code()));
                }
                // If only one instance is requested and recorded, we are done.
                if args.gemini_instances == 1 {
                    eprintln!("Session manager stage completed successfully.");
                    return Ok(());
                }
            }

            // Launch additional Gemini instances in the background if more than 1 are requested
            // and either no recording was requested or the first instance was already launched with asciinema.
            for i in 0..args.gemini_instances {
                if args.record_session && i == 0 {
                    // Skip the first instance if it was already launched with asciinema
                    continue;
                }
                eprintln!("Launching Gemini instance {}/{} in background...", i + 1, args.gemini_instances);
                let mut command = Command::new("gemini");
                command.arg("--model").arg("gemini-2.5-flash");
                command.arg("--checkpointing=true");
                command.stdout(Stdio::null()); // Redirect stdout to null to avoid clutter
                command.stderr(Stdio::null()); // Redirect stderr to null
                command.spawn()
                    .map_err(|e| format!("Failed to launch Gemini instance {}: {e}", i + 1))?;
            }

            eprintln!("Session manager stage completed successfully.");
            Ok(())
        },
        Commands::Export(args) => {
            eprintln!("Exporting session configuration to: {}", args.output);
            let config = LaunchArgs {
                gemini_instances: args.gemini_instances,
                record_session: args.record_session,
                background_detached: args.background_detached, // Include in export
            };
            let json_config = serde_json::to_string_pretty(&config)
                .map_err(|e| format!("Failed to serialize config to JSON: {e}"))?;

            let output_path = PathBuf::from(&args.output);
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create output directory {:?}: {e}", parent))?;
            }

            fs::write(&output_path, json_config)
                .map_err(|e| format!("Failed to write config to file {:?}: {e}", output_path))?;

            eprintln!("Session configuration exported successfully.");
            Ok(())
        }
    }
}
