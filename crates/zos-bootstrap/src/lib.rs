pub mod utils;
pub mod commands;
pub mod code_analysis;
pub mod constants;
pub mod cli;
pub mod command_handlers;

use crate::utils::error::Result;

pub fn main() -> Result<()> {
    command_handlers::handle_command_dispatch()
}