use tmux_interface::{Tmux, TmuxCommand, ListSessions, ListPanes};
use tokio::fs;
use uuid::Uuid;
use std::path::PathBuf;

pub async fn handle_capture_session_output_command() -> Result<(), Box<dyn std::error::Error>> {
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

            // Example pane_line: "0: [80x24] [history 0/1000, 1 active] (active)"
            // We need the pane ID, which is usually the first part before ':'
            let pane_id = pane_line.split(':').next().unwrap_or("").trim();
            if pane_id.is_empty() { continue; }

            println!("--- Capturing pane: {} in session: {} ---", pane_id, session_name);

            let unique_id = Uuid::new_v4().to_string();
            let session_pane_dir = session_store_base.join(session_name).join(pane_id);
            fs::create_dir_all(&session_pane_dir).await?;
            let temp_file_path = session_pane_dir.join(format!("capture_{}.txt", unique_id));
            let temp_file_path_str = temp_file_path.to_string_lossy().to_string();

            // Send command to tmux pane to capture its content to a file
            let capture_command_in_pane = format!("tmux capture-pane -p -t {} > {}", pane_id, temp_file_path_str);
            let mut send_keys_command = TmuxCommand::new();
            send_keys_command.name("send-keys");
            send_keys_command.push_param(&capture_command_in_pane);
            send_keys_command.push_param("Enter"); // Simulate pressing Enter
            send_keys_command.push_option("-t", pane_id); // Target the specific pane
            Tmux::with_command(send_keys_command).output()?;

            // Wait a moment for the file to be written (this is a heuristic and might need refinement)
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            // Read the content of the temporary file from the host
            let captured_content = fs::read_to_string(&temp_file_path).await?;
            println!("--- Captured content from pane {}:{} ---
{}", session_name, pane_id, captured_content);

            // Delete the temporary file
            fs::remove_file(&temp_file_path).await?;
        }
    }

    println!("--- Finished capturing output from all tmux sessions ---");
    Ok(())
}

