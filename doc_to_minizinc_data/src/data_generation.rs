use std::fs;
use crate::cli::Args;
use crate::logger::LogWriter;

// Declare sub-modules
pub mod initialize_data_structures;
pub mod process_files_and_collect_words;
pub mod write_data_declarations_mzn;
pub mod write_chunked_embeddings_dzn;
pub mod report_extracted_data;

// Re-export functions for easier access
pub use initialize_data_structures::initialize_data_structures;
pub use process_files_and_collect_words::process_files_and_collect_words;
pub use write_data_declarations_mzn::write_data_declarations_mzn;
pub use write_chunked_embeddings_dzn::write_chunked_embeddings_dzn;
pub use report_extracted_data::report_extracted_data;


pub fn generate_data(args: Args, all_relations: Vec<(String, String, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let minizinc_data_dir = current_dir.join("minizinc_data").join("huggingface");
    fs::create_dir_all(&minizinc_data_dir)?;

    let log_path = minizinc_data_dir.join("doc_to_minizinc_data.log");
    let mut logger = LogWriter::new(&log_path)?;

    logger.log(&format!("Generating data with args: {:?} and {} relations", args, all_relations.len()));

    // 1. Initialize data structures
    let mut initialized_data = initialize_data_structures();

    // 2. Process files and collect words
    let extensions = ["md", "rs", "cpp", "h", "hpp"];
    let input_path = if let Some(path) = args.input_path {
        path
    } else {
        current_dir
    };
    process_files_and_collect_words(
        &input_path,
        &extensions,
        &mut initialized_data.word_to_id,
        &mut initialized_data.id_to_word,
        &mut initialized_data.embeddings,
        &mut initialized_data.rng,
        &mut logger,
    )?;

    // 3. Write data_declarations.mzn
    write_data_declarations_mzn(
        &all_relations,
        &initialized_data.word_to_id,
        &initialized_data.id_to_word,
        &initialized_data.embeddings,
        &minizinc_data_dir,
        &mut logger,
    )?;

    // 4. Write chunked embeddings to .dzn files
    write_chunked_embeddings_dzn(
        &initialized_data.id_to_word,
        &initialized_data.word_to_id,
        &initialized_data.embeddings,
        &all_relations,
        args.chunk_size,
        &minizinc_data_dir,
        &mut logger,
    )?;

    // 5. Report extracted data
    report_extracted_data(
        &initialized_data.id_to_word,
        &initialized_data.embeddings,
        &mut logger,
    )?;

    Ok(())
}
