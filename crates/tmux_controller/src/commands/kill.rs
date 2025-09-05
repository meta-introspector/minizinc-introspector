use tmux_interface::{Tmux, KillSession, ListSessions};
use crate::commands::output_formatter;
use crate::commands::create::list_tmux_sessions_and_print;
use crate::error::TmuxControllerError;

pub async fn handle_kill_command(session_name: &str) -> Result<(), TmuxControllerError> {
    output_formatter::print_header(&format!("Killing tmux session: {}", session_name));
    let _ = Tmux::with_command(KillSession::new().target_session(session_name)).output();
    crate::commands::create::list_tmux_sessions_and_print("Current tmux sessions (after killing)").await?;
    Ok(())
}