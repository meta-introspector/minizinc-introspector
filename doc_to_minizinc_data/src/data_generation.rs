use std::fs;
use anyhow::Result;
use crate::cli::{Args, Command}; // Import Command enum
//use crate::app_config::AppConfig;
//use crate::AppConfig; // Import AppConfig directly
//use crate::prelude::LogWriter;

// Declare sub-modules
pub mod initialize_data_structures;
pub mod process_files_and_collect_words;
pub mod write_data_declarations_mzn;
pub mod write_chunked_embeddings_dzn;
pub mod report_extracted_data;
pub mod parquet_export; // New module

// Re-export functions for easier access
pub use initialize_data_structures::initialize_data_structures;
pub use process_files_and_collect_words::process_files_and_collect_words;
pub use write_data_declarations_mzn::write_data_declarations_mzn;
pub use write_chunked_embeddings_dzn::write_chunked_embeddings_dzn;
pub use report_extracted_data::report_extracted_data;
pub use parquet_export::export_embeddings_to_parquet; // New export
use crate::logger::LogWriter;
use serde::Deserialize; // Added for AppConfig
use std::path::PathBuf; // Added for AppConfig
use std::env; // Added for CARGO_MANIFEST_DIR
use std::collections::HashMap;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::{Array, StringArray, UInt32Array, Float64Array, ListArray};
use std::fs::File;
//use std::fs; // Added for AppConfig


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[allow(dead_code)]
    pub project_root: PathBuf,
    #[allow(dead_code)]
    pub github_root: PathBuf, // Not currently used in this module, but part of the overall configuration.
    #[allow(dead_code)]
    pub home_dir: PathBuf, // Not currently used in this module, but part of the overall configuration.
    #[allow(dead_code)]
    pub build_target: String, // Added build_target
    pub simulated_wordnet_path: PathBuf,
}

#[allow(dead_code)]
impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let config_path = manifest_dir.join("config.toml");
        let config_content = fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
}

//use crate::app_config::AppConfig;
//use crate::doc_to_minizinc_data::app_config::AppConfig;
#[allow(dead_code)]
fn get_all_relations_from_wordnet(config: &AppConfig) -> anyhow::Result<Vec<(String, String, f64)>> {
    let simulated_wordnet_path = &config.simulated_wordnet_path;
    let all_relations = crate::wordnet_processing::generate_wordnet_constraints(simulated_wordnet_path)?;
    Ok(all_relations)
}

fn load_embeddings_from_parquet(file_path: &PathBuf) -> Result<(HashMap<u32, String>, HashMap<u32, Vec<f64>>)> {
    let file = File::open(file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;

    let mut id_to_word: HashMap<u32, String> = HashMap::new();
    let mut id_to_embedding: HashMap<u32, Vec<f64>> = HashMap::new();

    for batch_result in reader {
        let record_batch = batch_result?;
        let id_column = record_batch.column_by_name("id").expect("id column not found");
        let word_column = record_batch.column_by_name("word").expect("word column not found");
        let embedding_column = record_batch.column_by_name("embedding").expect("embedding column not found");

        let id_array = id_column.as_any().downcast_ref::<UInt32Array>().expect("id column is not UInt32Array");
        let word_array = word_column.as_any().downcast_ref::<StringArray>().expect("word column is not StringArray");
        let embedding_list_array = embedding_column.as_any().downcast_ref::<ListArray>().expect("embedding column is not ListArray");

        for i in 0..id_array.len() {
            let id = id_array.value(i);
            let word = word_array.value(i).to_string();
            let embedding_values = embedding_list_array.value(i);
            let float_array = embedding_values.as_any().downcast_ref::<Float64Array>().expect("embedding list element is not Float64Array");
            let embedding_vec: Vec<f64> = float_array.values().to_vec();

            id_to_word.insert(id, word);
            id_to_embedding.insert(id, embedding_vec);
        }
    }
    Ok((id_to_word, id_to_embedding))
}

#[allow(dead_code)]
pub fn generate_data(args: Args, config: &AppConfig) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    // Extract chunk_size, input_path, and output_path from the Args struct
    let (chunk_size, input_path, output_path, previous_embeddings_path) = match args.command {
        Command::GenerateData { chunk_size, input_path, output_path, previous_embeddings_path } => (chunk_size, input_path, output_path, previous_embeddings_path),
        _ => return Err(anyhow::anyhow!("Invalid command for generate_data function")), // Should not happen if called correctly from main
    };

    fs::create_dir_all(&output_path)?;

    let log_path = output_path.join("doc_to_minizinc_data.log");
    let mut logger = LogWriter::new(&log_path)?;

    logger.debug_log("Starting generate_data function.");

    let mut fixed_id_to_word: HashMap<u32, String> = HashMap::new();
    let mut fixed_id_to_embedding: HashMap<u32, Vec<f64>> = HashMap::new();

    if let Some(path) = previous_embeddings_path {
        logger.debug_log(&format!("Loading previous embeddings from: {path:?}"));
        (fixed_id_to_word, fixed_id_to_embedding) = load_embeddings_from_parquet(&path)?;
        logger.debug_log(&format!("Loaded {} fixed embeddings.", fixed_id_to_word.len()));
    }

    let all_relations = get_all_relations_from_wordnet(config)?; // Get all_relations from new function

    logger.log(&format!("Generating data with chunk_size: {} and {} relations", chunk_size, all_relations.len()));

    logger.debug_log("1. Initializing data structures.");
    // 1. Initialize data structures
    let mut initialized_data = initialize_data_structures();
    logger.debug_log("1. Data structures initialized.");

    logger.debug_log("2. Processing files and collecting words.");
    // 2. Process files and collect words
    let extensions = ["md", "rs", "cpp", "h", "hpp"];
    let actual_input_path = if let Some(path) = input_path {
        path
    } else {
        current_dir
    };
    logger.debug_log(&format!("Attempting to process files from: {actual_input_path:?}"));
    process_files_and_collect_words(
        &actual_input_path,
        &extensions,
        &mut initialized_data.word_to_id,
        &mut initialized_data.id_to_word,
        &mut initialized_data.embeddings,
        &mut initialized_data.rng,
        &mut logger,
    )?;
    logger.debug_log(&format!("Successfully processed files from: {actual_input_path:?}"));
    logger.debug_log("2. Files processed and words collected.");

    logger.debug_log("3. Writing data_declarations.mzn.");
    // 3. Write data_declarations.mzn
    write_data_declarations_mzn(
        &all_relations,
        &initialized_data.word_to_id,
        &initialized_data.id_to_word,
        &initialized_data.embeddings,
        &output_path,
        &mut logger,
    )?;
    logger.debug_log("3. data_declarations.mzn written.");

    logger.debug_log("4. Writing chunked embeddings to .dzn files.");
    // 4. Write chunked embeddings to .dzn files
    write_chunked_embeddings_dzn(
        &initialized_data.id_to_word,
        &initialized_data.word_to_id,
        &initialized_data.embeddings,
        &all_relations,
        chunk_size, // Use the extracted chunk_size
        &output_path,
        &fixed_id_to_word,
        &fixed_id_to_embedding,
        &mut logger,
    )?;
    logger.debug_log("4. Chunked embeddings written to .dzn files.");

    logger.debug_log("5. Exporting embeddings to Parquet.");
    // 5. Export embeddings to Parquet
    export_embeddings_to_parquet(
        &initialized_data.id_to_word,
        &initialized_data.embeddings,
        &output_path,
    )?;
    logger.debug_log("5. Embeddings exported to Parquet.");

    logger.debug_log("6. Reporting extracted data.");
    // 6. Report extracted data
    report_extracted_data(
        &initialized_data.id_to_word,
        &initialized_data.embeddings,
        &mut logger,
    )?;
    logger.debug_log("6. Extracted data reported.");

    logger.debug_log("generate_data function finished successfully.");
    Ok(())
}
