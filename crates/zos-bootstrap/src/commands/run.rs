use clap::{Args, Subcommand};
use crate::utils::error::{Result, ZosError};
use crate::utils::subprocess;
use crate::utils::paths;
use std::path::PathBuf;
use std::fs;
use chrono; // For timestamp generation

// Declare new modules
pub mod run_embedding_model;
pub mod vector_params_source;
pub mod run_minimal_mzn;
pub mod run_test_driver; // Declare the new module

// Import the merged function and the new enum
use clap::{Args, Subcommand, Clone};
use crate::utils::error::{Result, ZosError};

pub mod run_embedding_model;
pub mod vector_params_source;
pub mod run_minimal_mzn;
pub mod run_test_driver;

use crate::commands::run::run_embedding_model::run_embedding_model;
use crate::commands::run::vector_params_source::VectorParamsSource;
use crate::commands::run::run_minimal_mzn::run_minimal_mzn;
use crate::commands::run::run_test_driver::run_test_driver;

#[derive(Args, Clone)]
pub struct RunArgs {
    #[command(subcommand)]
    pub command: Option<RunCommands>,
}

#[derive(Subcommand, Clone)]
pub enum RunCommands {
    /// Runs the embedding model with proof tape
    Embedding {
        main_model_version: String,
        core_params_version: String,
        kappa_params_version: String,
        other_params_version: String,
        relations_version: String,
        #[arg(long)]
        vector_params_version: Option<String>, // For static DZN file
        #[arg(long)]
        num_vec: Option<u32>, // For dynamic generation
    },
    /// Runs a MiniZinc model minimally (for quick debug)
    Minimal {
        main_model_version: String,
        core_params_version: String,
        kappa_params_version: String,
        other_params_version: String,
        relations_version: String,
        vector_params_version: String,
    },
    /// Runs the MiniZinc test driver (generates DZN, runs model)
    Driver {
        num_vec: u32,
        base_size: u32,
    },
}

pub fn handle_run_command(args: RunArgs) -> Result<()> {
    match args.command {
        Some(RunCommands::Embedding {
            main_model_version,
            core_params_version,
            kappa_params_version,
            other_params_version,
            relations_version,
            vector_params_version,
            num_vec,
        }) => {
            let vector_params_source = match (vector_params_version, num_vec) {
                (Some(version), None) => VectorParamsSource::Version(version),
                (None, Some(num)) => VectorParamsSource::NumVec(num),
                _ => return Err(ZosError::InvalidArgument(
                    "Either --vector-params-version or --num-vec must be provided, but not both.".to_string()
                )),
            };
            run_embedding_model(
                main_model_version,
                core_params_version,
                kappa_params_version,
                other_params_version,
                relations_version,
                vector_params_source,
            )?;
        }
        Some(RunCommands::Minimal {
            main_model_version,
            core_params_version,
            kappa_params_version,
            other_params_version,
            relations_version,
            vector_params_version,
        }) => {
            run_minimal_mzn(
                main_model_version,
                core_params_version,
                kappa_params_version,
                other_params_version,
                relations_version,
                vector_params_version,
            )?;
        }
        Some(RunCommands::Driver { num_vec, base_size }) => {
            run_test_driver(num_vec, base_size)?;
        }
        None => {
            println!("No run command provided. Use --help for more information.");
        }
    }
    Ok(())
}