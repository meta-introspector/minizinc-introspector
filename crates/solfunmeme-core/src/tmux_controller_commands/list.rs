use tmux_interface::{Tmux, ListSessions};
use crate::commands::output_formatter;

pub async fn handle_list_command() -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header("Current tmux sessions");
    let output = Tmux::with_command(ListSessions::new()).output()?;
    output_formatter::print_info(&String::from_utf8_lossy(&output.stdout()));
    output_formatter::print_footer();
    Ok(())
}