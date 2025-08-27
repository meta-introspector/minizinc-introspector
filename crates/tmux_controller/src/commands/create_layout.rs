use tmux_interface::{Tmux, TmuxCommand};
use clap::Args;
use crate::commands::output_formatter;

#[derive(Args, Debug)]
pub struct CreateLayoutArgs {
    /// Optional: A task to run in the middle pane (pane 1) after layout creation.
    #[arg(long)]
    pub task: Option<String>,
}

pub async fn handle_create_layout_command(args: &CreateLayoutArgs) -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header("Creating predefined tmux layout");

    // 1. Kill all other panes in the current window to start clean
    // This is a bit aggressive, but ensures a consistent starting point.
    // A more sophisticated approach might save/restore layouts.
    // This command might fail if there's only one pane, but that's okay.
    let mut kill_others_command = TmuxCommand::new();
    kill_others_command.name("kill-pane");
    kill_others_command.push_flag("-a"); // Kill all but the current pane
    // We don't check output here as it might fail if only one pane exists
    let _ = Tmux::with_command(kill_others_command).output();


    // 2. Split vertically to create three panes: work/data (0), gemini (1), and status (2).

    // Split for work/data pane (top) - creates pane 0 and pane 1
    let mut split_work_data_command = TmuxCommand::new();
    split_work_data_command.name("split-window");
    split_work_data_command.push_flag("-v"); // Vertical split
    Tmux::with_command(split_work_data_command).output()?;
    output_formatter::print_success("Window split for work/data pane.");

    // Select pane 1 (the large remaining one)
    let mut select_pane_1_command = TmuxCommand::new();
    select_pane_1_command.name("select-pane");
    select_pane_1_command.push_param("-t");
    select_pane_1_command.push_param("1"); // Target pane 1
    Tmux::with_command(select_pane_1_command).output()?;
    output_formatter::print_success("Selected pane 1 for further splitting.");

    // Split pane 1 into new pane 1 (gemini) and pane 2 (status)
    let mut split_gemini_status_command = TmuxCommand::new();
    split_gemini_status_command.name("split-window");
    split_gemini_status_command.push_flag("-v"); // Vertical split
    Tmux::with_command(split_gemini_status_command).output()?;
    output_formatter::print_success("Pane 1 split for Gemini (1) and Status (2) panes.");

    // Resize pane 1 (Gemini) to 3 lines
    let mut resize_gemini_command = TmuxCommand::new();
    resize_gemini_command.name("resize-pane");
    resize_gemini_command.push_param("-t");
    resize_gemini_command.push_param("1"); // Target pane 1
    resize_gemini_command.push_flag("-y"); // Resize vertically
    resize_gemini_command.push_param("3"); // Set height to 3 lines
    Tmux::with_command(resize_gemini_command).output()?;
    output_formatter::print_success("Gemini pane (1) resized to 3 lines.");

    // Resize pane 2 (Status) to 2 lines
    let mut resize_status_command = TmuxCommand::new();
    resize_status_command.name("resize-pane");
    resize_status_command.push_param("-t");
    resize_status_command.push_param("2"); // Target pane 2
    resize_status_command.push_flag("-y"); // Resize vertically
    resize_status_command.push_param("2"); // Set height to 2 lines
    Tmux::with_command(resize_status_command).output()?;
    output_formatter::print_success("Status pane (2) resized to 2 lines.");

    // Run launchpad-status in pane 2 (status pane)
    let mut select_pane_2_for_status_command = TmuxCommand::new();
    select_pane_2_for_status_command.name("select-pane");
    select_pane_2_for_status_command.push_param("-t");
    select_pane_2_for_status_command.push_param("2"); // Target pane 2
    Tmux::with_command(select_pane_2_for_status_command).output()?;
    output_formatter::print_success("Selected pane 2 for launchpad-status.");

    let mut send_status_command = TmuxCommand::new();
    send_status_command.name("send-keys");
    send_status_command.push_param("cargo run --package launchpad_status");
    send_status_command.push_param("C-m"); // Enter key
    Tmux::with_command(send_status_command).output()?;
    output_formatter::print_success("Running launchpad-status in pane 2.");

    // Run Gemini in pane 1 (middle pane)
    let mut select_pane_1_for_gemini_command = TmuxCommand::new();
    select_pane_1_for_gemini_command.name("select-pane");
    select_pane_1_for_gemini_command.push_param("-t");
    select_pane_1_for_gemini_command.push_param("1"); // Target pane 1
    Tmux::with_command(select_pane_1_for_gemini_command).output()?;
    output_formatter::print_success("Selected pane 1 for Gemini.");

    if let Some(task_name) = &args.task {
        let command_to_run = match task_name.as_str() {
            "crq-updater" => "cargo run --package crq_updater",
            "gemini-solfunmeme-dioxus" => "cd vendor/solfunmeme-dioxus && gemini",
            _ => {
                output_formatter::print_info(&format!("Unknown task: {}. Pane 1 will remain empty.", task_name));
                ""
            }
        };

        if !command_to_run.is_empty() {
            let mut send_task_command = TmuxCommand::new();
            send_task_command.name("send-keys");
            send_task_command.push_param("-t");
            send_task_command.push_param("1"); // Target pane 1
            send_task_command.push_param(command_to_run);
            send_task_command.push_param("C-m"); // Enter key
            Tmux::with_command(send_task_command).output()?;
            output_formatter::print_success(&format!("Launched task '{}' in pane 1.", task_name));
        }
    } else {
        output_formatter::print_success("Pane 1 left empty for native worker execution.");
    }

    // Select pane 0 (work/data pane) again for final cursor position
    let mut select_pane_0_final_command = TmuxCommand::new();
    select_pane_0_final_command.name("select-pane");
    select_pane_0_final_command.push_param("-t");
    select_pane_0_final_command.push_param("0"); // Target pane 0
    Tmux::with_command(select_pane_0_final_command).output()?;
    output_formatter::print_success("Selected pane 0 again for final cursor position.");


    output_formatter::print_success("Predefined tmux layout created successfully.");
    Ok(())
}