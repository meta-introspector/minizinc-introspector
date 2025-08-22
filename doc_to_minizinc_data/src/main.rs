use clap::Parser;
use crate::prelude::*; // Use the prelude
//use serde::Deserialize; // Added for AppConfig
//use std::path::PathBuf; // Added for AppConfig
//use std::fs; // Added for AppConfig

use crate::commands::run_hf_validator::handle_run_hf_validator_command;
// Explicit imports for command handlers
//use crate::commands::generate_data::handle_generate_data_command;
//use crate::commands::run_hf_validator::handle_run_hf_validator_command;
use crate::commands::inspect_parquet::handle_inspect_parquet_command;
use crate::commands::lookup_embedding::handle_lookup_embedding_command;
use crate::commands::inspect_parquet_schema::handle_inspect_parquet_schema_command;
use crate::commands::map_element_name_to_embedding::handle_map_element_name_to_embedding_command;

mod commands; // Declare the commands module
pub mod prelude; // Declare the prelude module
mod cli; // Declare the cli module
mod wordnet_processing; // Declare the wordnet_processing module
mod data_generation; // Declare the data_generation module
mod logger; // Declare the logger module
mod file_processing; // Declare the file_processing module
mod utils; // Declare the utils module
use doc_to_minizinc_data::data_generation::AppConfig;
use crate::commands::run_hf_validator::handle_generate_data_command;

#[allow(unused_variables)] // output_path is passed to handler, but not directly used in this scope
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load application configuration
    let config = AppConfig::load()?;

    match args.command {
        Command::GenerateData { chunk_size, ref input_path, ref output_path, ref previous_embeddings_path } => {
            println!("DEBUG: chunk_size = {chunk_size}");
            println!("DEBUG: input_path = {input_path:?}");
            // Pass config to handler
            handle_generate_data_command(args, &config)?; // Pass the original args and config
        },
        Command::RunHfValidator { project_path, output_path } => {
            // Pass config to handler
            handle_run_hf_validator_command(&config, project_path, output_path)?; // Pass config
        },
        Command::InspectParquet { file_path } => {
            // Pass config to handler if needed
            handle_inspect_parquet_command(file_path)?; // Pass original args
        },
        Command::LookupEmbedding { word } => {
            // Pass config to handler if needed
            handle_lookup_embedding_command(word)?; // Pass original args
        },
        Command::InspectParquetSchema { file_path } => {
            // Pass config to handler if needed
            handle_inspect_parquet_schema_command(file_path)?;
        },
        Command::MapElementNameToEmbedding { input_parquet_file, output_mapping_file } => {
            // Pass config to handler if needed
            handle_map_element_name_to_embedding_command(input_parquet_file, output_mapping_file)?;
        },
    }

    Ok(())
}




