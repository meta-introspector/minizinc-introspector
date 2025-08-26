use tmux_interface::{Tmux, TmuxCommand};

pub async fn handle_show_session_command(session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Splitting window horizontally and showing session: {} ---", session_name);

    // Split the current window horizontally
    let mut split_command = TmuxCommand::new();
    split_command.name("split-window");
    split_command.push_flag("-h");
    Tmux::with_command(split_command).output()?;

    // In the newly created pane, attach to the specified session
    // This assumes the new pane is the active one after splitting.
    let mut attach_command = TmuxCommand::new();
    attach_command.name("attach-session");
    attach_command.push_option("-t", session_name);
    Tmux::with_command(attach_command).output()?;

    println!("--- Window split and session shown successfully ---\n");
    Ok(())
}
