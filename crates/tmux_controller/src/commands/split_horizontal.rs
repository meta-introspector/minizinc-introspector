use tmux_interface::{Tmux, TmuxCommand};
use crate::commands::output_formatter;

pub async fn handle_split_horizontal_command() -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header("Splitting window horizontally");
    let mut tmux_command = TmuxCommand::new();
    tmux_command.name("split-window");
    tmux_command.push_flag("-h");
    Tmux::with_command(tmux_command).output()?;
    output_formatter::print_success("Window split successfully");
    Ok(())
}
