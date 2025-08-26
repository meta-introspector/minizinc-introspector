use tmux_interface::{Tmux, ListSessions};

pub async fn handle_list_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Current tmux sessions ---");
    let output = Tmux::with_command(ListSessions::new()).output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout()));
    println!("-----------------------------\
");
    Ok(())
}