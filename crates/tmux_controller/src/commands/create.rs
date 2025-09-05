use tmux_interface::{Tmux, NewSession, ListSessions, KillSession};
use crate::commands::output_formatter;

pub async fn handle_create_command(session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    list_tmux_sessions_and_print("Current tmux sessions (before creating new session)").await?;

    // Kill any existing session with the same name to ensure a clean start
    let _ = Tmux::with_command(KillSession::new().target_session(session_name)).output();

    // Create a new detached tmux session
    Tmux::with_command(
        NewSession::new()
            .detached()
            .session_name(session_name),
    )
    .output()?;

    list_tmux_sessions_and_print("Current tmux sessions (after creating new session)").await?;
    Ok(())
}

pub async fn list_tmux_sessions_and_print(header_message: &str) -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header(header_message);
    let output = Tmux::with_command(ListSessions::new()).output()?;
    output_formatter::print_info(&String::from_utf8_lossy(&output.stdout()));
    output_formatter::print_footer();
    Ok(())
}