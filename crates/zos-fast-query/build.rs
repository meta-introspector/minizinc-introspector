use std::env;
use std::path::PathBuf;

mod build_utils;
mod term_loader;
mod chunk_generator;
mod index_writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Rerun this build script if the hierarchical_term_index.json changes
    println!("cargo:rerun-if-changed={}", term_loader::HIERARCHICAL_INDEX_PATH);

    println!("cargo:warning=ZOS Fast Query Tool - Compiling Terms (Build Script)");

    let filtered_terms = term_loader::load_and_filter_terms()?;
    let out_dir = env::var("OUT_DIR")?;
    let out_dir_path = PathBuf::from(&out_dir);

    let (generated_files, _total_generated_size) = chunk_generator::generate_term_chunks(filtered_terms, &out_dir_path)?;
    index_writer::generate_recognizer_index(generated_files, &out_dir_path)?;

    Ok(())
}
