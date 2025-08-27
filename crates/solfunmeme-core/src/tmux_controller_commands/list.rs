use tmux_interface::{Tmux, ListSessions};
use super::output_formatter;
use gemini_utils::gemini_eprintln;

pub async fn handle_list_command() -> Result<(), Box<dyn std::error::Error>> {
    output_formatter::print_header("Current tmux sessions");
    let output = Tmux::with_command(ListSessions::new()).output()?;
    gemini_eprintln!("::output::", output = String::from_utf8_lossy(&output.stdout()));
    output_formatter::print_footer();
    Ok(())
}