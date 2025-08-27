use tmux_interface::{Tmux, TmuxCommand};
use super::output_formatter;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SplitVerticalArgs {
    /// Name of the tmux session to split
    #[arg(long)]
    pub session_name: Option<String>,
}

pub async fn handle_split_vertical_command(args: &SplitVerticalArgs) -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header("Splitting window vertically");
    let mut tmux_command = TmuxCommand::new();
    tmux_command.name("split-window");
    tmux_command.push_flag("-v");

    if let Some(session_name) = &args.session_name {
        tmux_command.push_param(format!("-t {}", session_name));
    }

    Tmux::with_command(tmux_command).output()?;
    output_formatter::print_success("Window split successfully");
    Ok(())
}
