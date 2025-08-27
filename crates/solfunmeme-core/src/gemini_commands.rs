use clap::Args;
use gemini_cli_manager::send_gemini_command;
use tokio::fs;
use crate::commands::output_formatter;

#[derive(Args, Debug)]
pub struct SendGeminiCommandArgs {
    /// Name of the tmux session where Gemini CLI is running
    #[arg(short, long)]
    pub session_name: String,
    /// Model to use for Gemini CLI (e.g., 'pro')
    #[arg(short, long)]
    pub model: Option<String>,
    /// Project directory for Gemini CLI (e.g., 'gemini-cli')
    #[arg(short, long)]
    pub project: Option<String>,
    /// Name of the CRQ file to send as a task (e.g., 'change_request_oauth_rust_module.md')
    #[arg(short, long)]
    pub crq: Option<String>,
}

pub async fn handle_gemini_command(args: &SendGeminiCommandArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut gemini_cli_command = "gemini".to_string();

    if let Some(model) = &args.model {
        gemini_cli_command.push_str(&format!(" --model {}", model));
    }

    let full_command = gemini_cli_command;

    output_formatter::print_header(&format!("Sending command to Gemini CLI in session: {}", args.session_name));
    output_formatter::print_info(&format!("Full command: {}", full_command));
    send_gemini_command(&args.session_name, &full_command).await?;

    if let Some(crq_name) = &args.crq {
        let crq_path = format!("/data/data/com.termux/files/home/storage/github/libminizinc/{}", crq_name);
        let crq_content = fs::read_to_string(&crq_path).await?;
        let instruction = format!("Please review the task outlined in the CRQ file: {} with the following content:\n\n```\n{}\n```\n\nand begin working on it.", crq_path, crq_content);
        output_formatter::print_header("Sending CRQ instruction to Gemini CLI");
        send_gemini_command(&args.session_name, &instruction).await?;
        output_formatter::print_success("CRQ instruction sent successfully");
    }

    output_formatter::print_success("Command sent successfully");
    Ok(())
}
