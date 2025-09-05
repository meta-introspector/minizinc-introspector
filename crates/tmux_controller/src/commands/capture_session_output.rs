use crate::commands::output_formatter;
use crate::commands::capture_utils;
use crate::error::TmuxControllerError;

pub async fn handle_capture_session_output_command(crq_number: Option<&str>) -> Result<(), TmuxControllerError> {
    output_formatter::print_header("Capturing output from all tmux sessions");

    let sessions_and_panes = capture_utils::get_all_sessions_and_panes().await.map_err(|e| TmuxControllerError::GenericError(e.to_string()))?;

    for (session_name, pane_id) in sessions_and_panes {
        output_formatter::print_info(&format!("Processing session: {}", session_name));
        output_formatter::print_info(&format!("Capturing pane: {} in session: {}", pane_id, session_name));

        let captured_content = capture_utils::capture_pane_content(&pane_id).await.map_err(|e| TmuxControllerError::GenericError(e.to_string()))?;
        let output_file_path = capture_utils::save_captured_content(
            &session_name,
            &pane_id,
            crq_number,
            &captured_content,
        ).await.map_err(|e| TmuxControllerError::GenericError(e.to_string()))?;

        output_formatter::print_info(&format!(
            "Captured content from pane {}:{} saved to {}",
            session_name,
            pane_id,
            output_file_path.display()
        ));
    }

    output_formatter::print_success("Finished capturing output from all tmux sessions");
    Ok(())
}