use clap::Parser;
use crate::prelude::*; // Use the prelude

// Explicit imports for command handlers and AppConfig
use crate::commands::generate_data::handle_generate_data_command;
use crate::commands::run_hf_validator::handle_run_hf_validator_command;
use crate::commands::inspect_parquet::handle_inspect_parquet_command;
use crate::commands::lookup_embedding::handle_lookup_embedding_command;
use crate::config::AppConfig;

mod commands; // Declare the commands module
mod config; // Declare the config module
pub mod prelude; // Declare the prelude module
mod cli; // Declare the cli module
mod wordnet_processing; // Declare the wordnet_processing module
mod data_generation; // Declare the data_generation module
mod logger; // Declare the logger module
mod file_processing; // Declare the file_processing module

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load application configuration
    let config = AppConfig::load()?;

    match args.command {
        Command::GenerateData { chunk_size, ref input_path } => {
            // Pass config to handler if needed, for now, just args
            handle_generate_data_command(args)?; // Pass the original args
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
    }

    Ok(())
}

// Helper function to recursively print Arrow schema fields (moved from main.rs)
fn print_arrow_schema_fields(schema: &Schema, indent_level: usize) {
    let indent = "  ".repeat(indent_level);
    for field in schema.fields() {
        println!("{}  - Name: {}, Data Type: {:?}", indent, field.name(), field.data_type());

        match field.data_type() {
            DataType::List(field_arc) => {
                println!("{}    List Element:", indent);
                print_arrow_schema_fields(&Schema::new(vec![field_arc.as_ref().clone()]), indent_level + 1);
            },
            DataType::Struct(fields_vec) => {
                println!("{}    Struct Fields:", indent);
                print_arrow_schema_fields(&Schema::new(fields_vec.clone()), indent_level + 1);
            },
            _ => {
                // No children for other data types
            }
        }
    }
}