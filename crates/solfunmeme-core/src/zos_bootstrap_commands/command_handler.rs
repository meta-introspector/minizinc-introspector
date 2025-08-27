use crate::zos_bootstrap_commands::Cli;
use crate::zos_bootstrap_commands::Commands;
use crate::zos_bootstrap_commands::build;

pub async fn handle_zos_bootstrap_command(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = &cli.command {
        match command {
            Commands::Build(args) => {
                build::handle_build_command(args.clone())?;
            }
            _ => {
                println!("This command is not yet implemented.");
            }
        }
    }
    Ok(())
}