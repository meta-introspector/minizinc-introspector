use tmux_interface::{Tmux, SendKeys};

pub async fn send_gemini_command(
    session_name: &str,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_pane = format!("{}:0.0", session_name); // Assuming window 0, pane 0

    // Send the command to the tmux pane
    Tmux::with_command(
        SendKeys::new()
            .target_pane(&target_pane)
            .key(command)
            .key("Enter"), // Simulate pressing Enter after the command
    )
    .output()?;

    Ok(())
}