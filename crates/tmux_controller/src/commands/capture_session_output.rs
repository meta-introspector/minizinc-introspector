use tmux_interface::{Tmux, TmuxCommand, ListSessions, ListPanes, CapturePane};
use tokio::fs;
use uuid::Uuid;
use std::path::PathBuf;
use chrono::Local;

pub async fn handle_capture_session_output_command(crq_number: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Capturing output from all tmux sessions ---");

    let sessions_output = Tmux::with_command(ListSessions::new()).output()?;
    let sessions_str = String::from_utf8_lossy(&sessions_output.stdout()).to_string();
    let session_lines: Vec<&str> = sessions_str.lines().collect();

    let project_root = PathBuf::from(std::env::var("PWD").unwrap_or_else(|_| ".".to_string()));
    let session_store_base = project_root.join("sessions");

    for session_line in session_lines {
        if session_line.is_empty() { continue; }

        let session_name = session_line.split(':').next().unwrap_or("").trim();
        if session_name.is_empty() { continue; }

        println!("--- Processing session: {} ---", session_name);

        let panes_output = Tmux::with_command(ListPanes::new().target(session_name)).output()?;
        let panes_str = String::from_utf8_lossy(&panes_output.stdout()).to_string();
        let pane_lines: Vec<&str> = panes_str.lines().collect();

        for pane_line in pane_lines {
            if pane_line.is_empty() { continue; }

            let pane_id = pane_line.split(':').next().unwrap_or("").trim();
            if pane_id.is_empty() { continue; }

            println!("--- Capturing pane: {} in session: {} ---", pane_id, session_name);

            // Execute tmux capture-pane directly from the host
            let capture_pane_command = CapturePane::new().target_pane(pane_id).stdout(); // -p flag
            let captured_output = Tmux::with_command(capture_pane_command).output()?;
            let captured_content = String::from_utf8_lossy(&captured_output.stdout()).to_string();

            // Store the captured content in the structured session store
            let now = Local::now();
            let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
            let crq_prefix = if let Some(crq) = crq_number { format!("crq-{}-", crq) } else { String::new() };
            let filename = format!("{}{}_{}_capture.txt", crq_prefix, session_name, timestamp);

            let session_pane_dir = session_store_base.join(session_name).join(pane_id);
            fs::create_dir_all(&session_pane_dir).await?;
            let output_file_path = session_pane_dir.join(filename);

            fs::write(&output_file_path, captured_content).await?;
            println!("--- Captured content from pane {}:{} saved to {} ---", session_name, pane_id, output_file_path.display());
        }
    }

    println!("--- Finished capturing output from all tmux sessions ---");
    Ok(())
}


