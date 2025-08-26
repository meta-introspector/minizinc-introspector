use clap::{Args, Parser, Subcommand};
use gemini_cli_manager::send_gemini_command;

#[derive(Args, Debug)]
pub struct SendGeminiCommandArgs {
    /// Name of the tmux session where Gemini CLI is running
    #[arg(short, long)]
    pub session_name: String,
    /// Command to send to Gemini CLI
    #[arg(short, long)]
    pub command: String,
}

pub async fn handle_gemini_command(args: &SendGeminiCommandArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Sending command to Gemini CLI in session: {} ---", args.session_name);
    send_gemini_command(&args.session_name, &args.command).await?;
    println!("--- Command sent successfully ---");
    Ok(())
}
