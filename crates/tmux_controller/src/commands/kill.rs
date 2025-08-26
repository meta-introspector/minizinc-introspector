use tmux_interface::{Tmux, KillSession, ListSessions};

pub async fn handle_kill_command(session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Killing tmux session: {} --", session_name);
    let _ = Tmux::with_command(KillSession::new().target_session(session_name)).output();
    println!("-- Current tmux sessions (after killing) --");
    let output = Tmux::with_command(ListSessions::new()).output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout()));
    println!("---------------------------------------------\n");
    Ok(())
}
