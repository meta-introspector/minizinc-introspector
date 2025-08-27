use tmux_interface::{Tmux, ListSessions, ListPanes, CapturePane};
use tokio::fs;
use std::path::PathBuf;
use chrono::Local;
use std::error::Error;

pub async fn get_all_sessions_and_panes() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut sessions_and_panes = Vec::new();

    let sessions_output = Tmux::with_command(ListSessions::new()).output()?;
    let sessions_str = String::from_utf8_lossy(&sessions_output.stdout()).to_string();
    let session_lines: Vec<&str> = sessions_str.lines().collect();

    for session_line in session_lines {
        if session_line.is_empty() { continue; }

        let session_name = session_line.split(':').next().unwrap_or("").trim();
        if session_name.is_empty() { continue; }

        let panes_output = Tmux::with_command(ListPanes::new().target(session_name)).output()?;
        let panes_str = String::from_utf8_lossy(&panes_output.stdout()).to_string();
        let pane_lines: Vec<&str> = panes_str.lines().collect();

        for pane_line in pane_lines {
            if pane_line.is_empty() { continue; }
            let pane_id = pane_line.split(':').next().unwrap_or("").trim();
            if pane_id.is_empty() { continue; }
            sessions_and_panes.push((session_name.to_string(), pane_id.to_string()));
        }
    }
    Ok(sessions_and_panes)
}

pub async fn capture_pane_content(pane_id: &str) -> Result<String, Box<dyn Error>> {
    let capture_pane_command = CapturePane::new().target_pane(pane_id).stdout();
    let captured_output = Tmux::with_command(capture_pane_command).output()?;
    Ok(String::from_utf8_lossy(&captured_output.stdout()).to_string())
}

pub async fn save_captured_content(
    session_name: &str,
    pane_id: &str,
    crq_number: Option<&str>,
    content: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let project_root = PathBuf::from(std::env::var("PWD").unwrap_or_else(|_| ".".to_string()));
    let session_store_base = project_root.join("sessions");

    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
    let crq_prefix = if let Some(crq) = crq_number { format!("crq-{}-", crq) } else { String::new() };
    let filename = format!("{}{}_{}_capture.txt", crq_prefix, session_name, timestamp);

    let session_pane_dir = session_store_base.join(session_name).join(pane_id);
    fs::create_dir_all(&session_pane_dir).await?;
    let output_file_path = session_pane_dir.join(filename);

    fs::write(&output_file_path, content).await?;
    Ok(output_file_path)
}
