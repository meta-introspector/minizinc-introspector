use clap::Subcommand;
//use serde::{Serialize, Deserialize};

pub mod launch;
pub mod export;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Launches a new session
    #[command(disable_help_flag = true)]
    Launch { serialized_args: String },
    /// Exports the current session configuration to a file
    Export(export::ExportArgs),
}
