use tmux_interface::{Tmux, TmuxCommand};
use std::process::Command;
use std::path::PathBuf;
use chrono::{Local, Timelike}; // For timestamping log files

#[derive(Debug, clap::Args)]
pub struct TmuxViewArgs {
    /// Optional CRQ number to include in the capture filename.
    #[arg(short, long)]
    pub crq_number: Option<String>,
    /// Optional session name to target. If not provided, all sessions are processed.
    #[arg(short, long)]
    pub session_name: Option<String>,
    /// Path to store the captured session output. Defaults to sessions/
    #[arg(short, long, default_value = "sessions/")]
    pub output_path: PathBuf,
}

pub async fn handle_tmux_view_command(args: &TmuxViewArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Tmux Sessions --");
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

    println!("\n--- Capturing all pane output via tmux_controller ---");
    let mut capture_cmd = Command::new("cargo");
    capture_cmd.arg("run")
        .arg("--package")
        .arg("tmux_controller")
        .arg("--")
        .arg("capture-session-output");

    if let Some(crq) = &args.crq_number {
        capture_cmd.arg("--crq-number").arg(crq);
    }
    if let Some(session) = &args.session_name {
        capture_cmd.arg("--session-name").arg(session);
    }

    let output_capture = capture_cmd.output()?;
    println!("{}", String::from_utf8_lossy(&output_capture.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output_capture.stderr)); // Print stderr for debugging

    // After capture, the files are in sessions/<session_id>/<pane_id>/...
    // We would then need to read those files and extract the last page.
    // For simplicity, I'll just list the generated files and instruct the user to check them.
    println!("\n--- Captured pane output files (check these for last page) ---");
    let output_ls_sessions = Command::new("ls")
        .arg("-R")
        .arg(&args.output_path)
        .output()?;
    println!("{}", String::from_utf8_lossy(&output_ls_sessions.stdout));


    Ok(())
}
