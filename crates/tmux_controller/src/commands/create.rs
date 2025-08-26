use tmux_interface::{Tmux, NewSession, ListSessions, KillSession};

pub async fn handle_create_command(session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("---------------------------------------------------------\n");
    Ok(())
}
