use clap::{Parser, Subcommand};

mod gemini_commands;
mod commands;
use commands::{split_vertical, split_horizontal, create_layout};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new tmux session
    Create {
        /// Name of the new tmux session
        #[arg(short, long)]
        session_name: String,
    },
    /// Lists all active tmux sessions
    List,
    /// Kills a specified tmux session
    Kill {
        /// Name of the tmux session to kill
        #[arg(short, long)]
        session_name: String,
    },
    /// Manages Gemini CLI interactions within tmux sessions
    Gemini(gemini_commands::SendGeminiCommandArgs),
    /// Sends a raw tmux command to a session
    SendCommand {
        /// Name of the tmux session to send the command to (defaults to current)
        #[arg(short, long)]
        session_name: Option<String>,
        /// The tmux command to send (e.g., 'split-window -h')
        #[arg(short, long)]
        command: String,
    },
    /// Splits the current tmux window vertically
    SplitVertical(split_vertical::SplitVerticalArgs),
    /// Splits the current tmux window horizontally
    SplitHorizontal(split_horizontal::SplitHorizontalArgs),
    /// Selects and displays a specific tmux session
    SelectSession {
        /// Name of the tmux session to select
        #[arg(short, long)]
        session_name: String,
    },
    /// Splits the current window and shows the specified session in the new pane
    ShowSession {
        /// Name of the session to show
        #[arg(short, long)]
        session_name: String,
    },
    /// Captures and reports the textual output from all panes in all active tmux sessions.
    CaptureSessionOutput {
        /// Optional CRQ number to include in the capture filename.
        #[arg(short, long)]
        crq_number: Option<String>,
    },
    /// Creates a predefined tmux layout (e.g., one large pane, one small pane)
    CreateLayout,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { session_name } => {
            commands::create::handle_create_command(session_name).await?;
        },
        Commands::List => {
            commands::list::handle_list_command().await?;
        },
        Commands::Kill { session_name } => {
            commands::kill::handle_kill_command(session_name).await?;
        },
        Commands::Gemini(gemini_command) => {
            gemini_commands::handle_gemini_command(gemini_command).await?;
        },
        Commands::SendCommand { session_name, command } => {
            commands::send_command::handle_send_command(session_name.as_deref(), command).await?;
        },
        Commands::SplitVertical(args) => {
            commands::split_vertical::handle_split_vertical_command(args).await?;
        },
        Commands::SplitHorizontal(args) => {
            commands::split_horizontal::handle_split_horizontal_command(args).await?;
        },
        Commands::SelectSession { session_name } => {
            commands::select_session::handle_select_session_command(session_name).await?;
        },
        Commands::ShowSession { session_name } => {
            commands::show_session::handle_show_session_command(session_name).await?;
        },
        Commands::CaptureSessionOutput { crq_number } => {
            commands::capture_session_output::handle_capture_session_output_command(crq_number.as_deref()).await?;
        },
        Commands::CreateLayout => {
            commands::create_layout::handle_create_layout_command().await?;
        },
    }

    Ok(())
}