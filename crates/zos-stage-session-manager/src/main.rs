use clap::Parser;

mod commands;
mod utils;
mod args; // Declare the new args module

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        commands::Commands::Launch { serialized_args } => commands::launch::handle_launch_command(serialized_args),
        commands::Commands::Export(args) => commands::export::handle_export_command(args),
    }
}
