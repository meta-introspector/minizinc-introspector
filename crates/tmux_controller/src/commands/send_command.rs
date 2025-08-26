use tmux_interface::{Tmux, TmuxCommand};

pub async fn handle_send_command(session_name: Option<&str>, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Sending tmux command: '{}' ---", command);
    let mut tmux_command = TmuxCommand::new();
    tmux_command.name("send-keys");
    tmux_command.push_param(command);

    if let Some(s_name) = session_name {
        tmux_command.push_option("-t", s_name);
    }

    Tmux::with_command(tmux_command).output()?;
    println!("--- Tmux command sent successfully ---\n");
    Ok(())
}
