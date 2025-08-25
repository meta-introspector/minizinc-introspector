use tmux_interface::{Tmux, NewSession, SendKeys, CapturePane, KillSession};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session_name = "test_session";
    let window_name = "test_window";
    let pane_target = format!("{}:{}", session_name, window_name);

    // 1. Kill any existing session with the same name to ensure a clean start
    let _ = Tmux::with_command(KillSession::new().target_session(session_name)).output();

    // 2. Create a new detached tmux session and window
    Tmux::with_command(
        NewSession::new()
            .detached()
            .session_name(session_name)
            .window_name(window_name),
    )
    .output()?;

    // 3. Send the command to run zos-stage-process-monitor in the new pane
    //    We need to send the full path to the executable.
    let zos_binary_path = "target/debug/zos-stage-process-monitor";
    Tmux::with_command(
        SendKeys::new()
            .target_pane(&pane_target)
            .key(zos_binary_path)
            .key("Enter"),
    )
    .output()?;

    // 4. Wait for the TUI to initialize and render its content
    sleep(Duration::from_secs(2)).await; // Adjust this duration if needed

    // 5. Capture the pane content
    let output = Tmux::with_command(
        CapturePane::new()
            .target_pane(&pane_target)
            .start_line("-") // Capture from the beginning of history
            .end_line("~") // Capture to the end of the current screen
            .stdout(), // Pipe to stdout
    )
    .output()?;

    println!("--- Captured tmux pane output ---");
    println!("{}", String::from_utf8_lossy(&output.stdout()));
    println!("---------------------------------");

    // 6. Kill the tmux session
    Tmux::with_command(KillSession::new().target_session(session_name)).output()?;

    Ok(())
}
