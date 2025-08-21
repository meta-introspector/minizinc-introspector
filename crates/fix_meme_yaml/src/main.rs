use std::{
    //path::PathBuf,
    collections::HashMap};
use anyhow::Result;
use clap::Parser;
use walkdir::WalkDir;

use fix_meme_yaml::file_processing;
use fix_meme_yaml::patching;

// Command-line arguments struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional: Name of a Rust function to use for patching YAML errors.
    #[arg(long)]
    patch_function: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut patch_functions: HashMap<String, fn(&str) -> String> = HashMap::new();
    patch_functions.insert("quote_line".to_string(), patching::patch_quote_line);
    // Add other patch functions to the map here

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Processing: {:?}\n", path);
            // Fix: Convert &Path to PathBuf and then pass a reference
            let path_buf = path.to_path_buf(); // Convert &Path to PathBuf
            file_processing::process_poem_file(&path_buf, &args.patch_function, &patch_functions)?;
        }
    }

    Ok(())
}
