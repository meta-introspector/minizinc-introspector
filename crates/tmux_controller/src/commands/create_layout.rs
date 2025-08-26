use tmux_interface::{Tmux, TmuxCommand};
use crate::commands::output_formatter;

pub async fn handle_create_layout_command() -> Result<(), Box<dyn std::error::Error>> {
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


    // 2. Split vertically to create two panes
    let mut split_command = TmuxCommand::new();
    split_command.name("split-window");
    split_command.push_flag("-v"); // Vertical split
    Tmux::with_command(split_command).output()?;
    output_formatter::print_success("Window split vertically.");

    // 3. Resize panes to create one large and one small pane
    // Assuming the active pane is the upper one after the split.
    // Make the upper pane 70% height, lower pane 30%.
    let mut resize_command = TmuxCommand::new();
    resize_command.name("resize-pane");
    resize_command.push_flag("-y"); // Resize vertically
    resize_command.push_param("70%"); // Set height to 70%
    Tmux::with_command(resize_command).output()?;
    output_formatter::print_success("Panes resized to 70/30 split.");

    // 4. Select the large pane (upper one)
    let mut select_pane_command = TmuxCommand::new();
    select_pane_command.name("select-pane");
    select_pane_command.push_flag("-U"); // Select upper pane
    Tmux::with_command(select_pane_command).output()?;
    output_formatter::print_success("Selected large pane.");


    output_formatter::print_success("Predefined tmux layout created successfully.");
    Ok(())
}