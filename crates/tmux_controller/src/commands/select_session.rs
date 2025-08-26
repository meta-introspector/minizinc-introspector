use tmux_interface::{Tmux, TmuxCommand};
use crate::commands::output_formatter;

pub async fn handle_select_session_command(session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header(&format!("Selecting tmux session: {}", session_name));
    let mut tmux_command = TmuxCommand::new();
    tmux_command.name("switch-client");
    tmux_command.push_option("-t", session_name);
    Tmux::with_command(tmux_command).output()?;
    output_formatter::print_success("Session selected successfully");
    Ok(())
}
