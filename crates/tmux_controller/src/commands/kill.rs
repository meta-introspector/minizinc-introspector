use tmux_interface::{Tmux, KillSession, ListSessions};
use crate::commands::output_formatter;

pub async fn handle_kill_command(session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header(&format!("Killing tmux session: {}", session_name));
    let _ = Tmux::with_command(KillSession::new().target_session(session_name)).output();
    output_formatter::print_header("Current tmux sessions (after killing)");
    let output = Tmux::with_command(ListSessions::new()).output()?;
    output_formatter::print_info(&String::from_utf8_lossy(&output.stdout()));
    output_formatter::print_footer("");
    Ok(())
}
