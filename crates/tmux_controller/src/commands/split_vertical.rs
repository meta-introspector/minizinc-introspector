use tmux_interface::{Tmux, TmuxCommand};

pub async fn handle_split_vertical_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Splitting window vertically ---");
    let mut tmux_command = TmuxCommand::new();
    tmux_command.name("split-window");
    tmux_command.push_flag("-v");
    Tmux::with_command(tmux_command).output()?;
    println!("--- Window split successfully ---\n");
    Ok(())
}
