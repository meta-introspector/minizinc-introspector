use tmux_interface::{Tmux, TmuxCommand};
use super::output_formatter;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SplitHorizontalArgs {
    /// Name of the tmux session to split
    #[arg(long)]
    pub session_name: Option<String>,
}

pub async fn handle_split_horizontal_command(args: &SplitHorizontalArgs) -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header("Splitting window horizontally");
    let mut tmux_command = TmuxCommand::new();
    tmux_command.name("split-window");
    tmux_command.push_flag("-h");

    if let Some(session_name) = &args.session_name {
        tmux_command.push_param(format!("-t {}", session_name));
    }

    Tmux::with_command(tmux_command).output()?;
    output_formatter::print_success("Window split successfully");
    Ok(())
}
