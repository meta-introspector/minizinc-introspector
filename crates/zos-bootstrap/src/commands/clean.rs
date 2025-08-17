use clap::{Args, Subcommand};
use crate::utils::error::Result;
use crate::utils::paths;
use std::fs;
use crate::commands::build_constants;

#[derive(Args, Clone)]
pub struct CleanArgs {
    #[command(subcommand)]
    pub command: Option<CleanCommands>,
}

#[derive(Subcommand, Clone)]
pub enum CleanCommands {
    /// Cleans all build artifacts
    All {},
    /// Cleans the main build directory
    Build {},
    /// Cleans the coverage build directory
    Coverage {},
    /// Cleans generated proof tapes
    ProofTapes {},
}

pub fn handle_clean_command(args: CleanArgs) -> Result<()> {
    match args.command {
        Some(CleanCommands::All {}) => {
            println!("Cleaning all build artifacts...");
            clean_build_dir()?;
            clean_coverage_dir()?;
            clean_proof_tapes()?;
            println!("All artifacts cleaned.");
        }
        Some(CleanCommands::Build {}) => {
            clean_build_dir()?;
        }
        Some(CleanCommands::Coverage {}) => {
            clean_coverage_dir()?;
        }
        Some(CleanCommands::ProofTapes {}) => {
            clean_proof_tapes()?;
        }
        None => {
            println!("No clean command provided. Use --help for more information.");
        }
    }
    Ok(())
}

fn clean_build_dir() -> Result<()> {
    println!("Cleaning main build directory...");
    let build_dir = paths::get_build_dir()?;
    if build_dir.exists() {
        fs::remove_dir_all(&build_dir)?;
        println!("Removed: {}", build_dir.to_string_lossy());
    }
    Ok(())
}

fn clean_coverage_dir() -> Result<()> {
    println!("Cleaning coverage build directory...");
    let build_coverage_dir = paths::get_build_dir()?.join(build_constants::COVERAGE_DIR_SUFFIX);
    if build_coverage_dir.exists() {
        fs::remove_dir_all(&build_coverage_dir)?;
        println!("Removed: {}", build_coverage_dir.to_string_lossy());
    }
    Ok(())
}

fn clean_proof_tapes() -> Result<()> {
    println!("Cleaning generated proof tapes...");
    let proof_tapes_dir = paths::get_proof_tapes_dir()?;
    if proof_tapes_dir.exists() {
        fs::remove_dir_all(&proof_tapes_dir)?;
        println!("Removed: {}", proof_tapes_dir.to_string_lossy());
    }
    Ok(())
}
