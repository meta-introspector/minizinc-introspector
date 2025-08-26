use clap::{Parser, Subcommand};
use tmux_interface::{Tmux, NewSession, ListSessions, KillSession};

mod gemini_commands;

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
    #[command(subcommand)]
    Gemini(gemini_commands::Commands),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { session_name } => {
            println!("--- Current tmux sessions (before creating new session) ---");
            let output_before = Tmux::with_command(ListSessions::new()).output()?;
            println!("{}", String::from_utf8_lossy(&output_before.stdout()));
            println!("----------------------------------------------------------");

            // Kill any existing session with the same name to ensure a clean start
            let _ = Tmux::with_command(KillSession::new().target_session(session_name)).output();

            // Create a new detached tmux session
            Tmux::with_command(
                NewSession::new()
                    .detached()
                    .session_name(session_name),
            )
            .output()?;

            println!("--- Current tmux sessions (after creating new session) ---");
            let output_after = Tmux::with_command(ListSessions::new()).output()?;
            println!("{}", String::from_utf8_lossy(&output_after.stdout()));
            println!("---------------------------------------------------------");
        },
        Commands::List => {
            println!("--- Current tmux sessions ---");
            let output = Tmux::with_command(ListSessions::new()).output()?;
            println!("{}", String::from_utf8_lossy(&output.stdout()));
            println!("-----------------------------");
        },
        Commands::Kill { session_name } => {
            println!("--- Killing tmux session: {} ---", session_name);
            let _ = Tmux::with_command(KillSession::new().target_session(session_name)).output();
            println!("--- Current tmux sessions (after killing) ---");
            let output = Tmux::with_command(ListSessions::new()).output()?;
            println!("{}", String::from_utf8_lossy(&output.stdout()));
            println!("---------------------------------------------");
        },
        Commands::Gemini(gemini_command) => {
            gemini_commands::handle_gemini_command(gemini_command).await?;
        },
    }

    Ok(())
}
