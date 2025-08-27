use clap::{Parser, Subcommand};

// Re-export the individual command modules
pub mod capture_session_output;
pub mod capture_utils;
pub mod create;
pub mod kill;
pub mod list;
pub mod output_formatter;
pub mod select_session;
pub mod send_command;
pub mod show_session;
pub mod split_horizontal;
pub mod split_vertical;
pub mod create_layout;
pub mod tmux_view;

// Re-export gemini_commands for use in the main Cli
use crate::gemini_commands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
    CreateLayout(create_layout::CreateLayoutArgs),
    /// Provides an overview of the current tmux state, including pane content.
    TmuxView(tmux_view::TmuxViewArgs),
}

pub async fn handle_cli_command(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Create { session_name } => {
            create::handle_create_command(session_name).await?;
        },
        Commands::List => {
            list::handle_list_command().await?;
        },
        Commands::Kill { session_name } => {
            kill::handle_kill_command(session_name).await?;
        },
        Commands::Gemini(gemini_command) => {
            gemini_commands::handle_gemini_command(gemini_command).await?;
        },
        Commands::SendCommand { session_name, command } => {
            send_command::handle_send_command(session_name.as_deref(), command).await?;
        },
        Commands::SplitVertical(args) => {
            split_vertical::handle_split_vertical_command(args).await?;
        },
        Commands::SplitHorizontal(args) => {
            split_horizontal::handle_split_horizontal_command(args).await?;
        },
        Commands::SelectSession { session_name } => {
            select_session::handle_select_session_command(session_name).await?;
        },
        Commands::ShowSession { session_name } => {
            show_session::handle_show_session_command(session_name).await?;
        },
        Commands::CaptureSessionOutput { crq_number } => {
            capture_session_output::handle_capture_session_output_command(crq_number.as_deref()).await?;
        },
        Commands::CreateLayout(args) => {
            create_layout::handle_create_layout_command(args).await?;
        },
        Commands::TmuxView(args) => {
            tmux_view::handle_tmux_view_command(args).await?;
        },
    }
    Ok(())
}