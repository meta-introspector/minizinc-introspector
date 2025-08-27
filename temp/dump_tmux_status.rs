use tmux_interface::{Tmux, TmuxCommand};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Tmux Sessions ---");
    let output_ls = Command::new("tmux")
        .arg("ls")
        .output()?;
    println!("{}", String::from_utf8_lossy(&output_ls.stdout));

    println!("\n--- All Tmux Panes (across all sessions) ---");
    let output_list_panes = Command::new("tmux")
        .arg("list-panes")
        .arg("-a") // List all panes across all sessions
        .arg("-F") // Format output
        .arg("#{session_name}:#{window_index}.#{pane_index} #{pane_current_command} #{pane_current_path}") // Custom format
        .output()?;
    println!("{}", String::from_utf8_lossy(&output_list_panes.stdout));

    // To get the last page of output from all panes, we can reuse tmux_controller's capture functionality
    println!("\n--- Capturing all pane output via tmux_controller ---");
    let output_capture = Command::new("cargo")
        .arg("run")
        .arg("--package")
        .arg("tmux_controller")
        .arg("--")
        .arg("capture-session-output")
        .output()?;
    println!("{}", String::from_utf8_lossy(&output_capture.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output_capture.stderr)); // Print stderr for debugging

    // After capture, the files are in sessions/<session_id>/<pane_id>/...
    // We would then need to read those files and extract the last page.
    // For simplicity, I'll just list the generated files and instruct the user to check them.
    println!("\n--- Captured pane output files (check these for last page) ---");
    let output_ls_sessions = Command::new("ls")
        .arg("-R")
        .arg("sessions/")
        .output()?;
    println!("{}", String::from_utf8_lossy(&output_ls_sessions.stdout));


    Ok(())
}
