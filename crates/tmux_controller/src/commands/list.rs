use tmux_interface::{Tmux, ListSessions};
use crate::commands::output_formatter;
use crate::commands::create::list_tmux_sessions_and_print;

pub async fn handle_list_command() -> Result<(), Box<dyn std::error::Error>> {
    list_tmux_sessions_and_print("Current tmux sessions").await?;
    Ok(())
}