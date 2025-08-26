use tmux_interface::{Tmux, TmuxCommand};

pub async fn handle_select_session_command(session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Selecting tmux session: {} ---", session_name);
    let mut tmux_command = TmuxCommand::new();
    tmux_command.name("switch-client");
    tmux_command.push_option("-t", session_name);
    Tmux::with_command(tmux_command).output()?;
    println!("--- Session selected successfully ---\n");
    Ok(())
}
