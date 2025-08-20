use std::env;
use std::path::PathBuf;
use serde::Deserialize;
use std::fs;

pub fn sanitize_filename_char(c: char) -> String {
    if c.is_ascii_alphanumeric() {
        c.to_string()
    } else {
        format!("U{:04X}", c as u32) // Format as UXXXX (hex Unicode codepoint)
    }
}

pub fn sanitize_filename(s: &str) -> String {
    s.chars().map(|c| sanitize_filename_char(c)).collect::<Vec<String>>().join("")
}


#[derive(Debug, Deserialize)]
struct ConfigWrapper {
    paths: BuildPathsConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BuildPathsConfig {
    #[allow(dead_code)]
    home_dir: String,
    github_root: String,
    project_root: String,
    hierarchical_term_index: String,
}


mod term_loader;
mod chunk_generator;
mod index_writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/build_config.toml");
    let config_content = fs::read_to_string(&config_path)?;
    let config: ConfigWrapper = toml::from_str(&config_content)?;

    // Rerun this build script if the build_config.toml changes
    println!("cargo:rerun-if-changed={}", config_path.display());

    println!("cargo:warning=ZOS Fast Query Tool - Compiling Terms (Build Script)");

    let filtered_terms = term_loader::load_and_filter_terms(&config.paths.hierarchical_term_index)?;
    let out_dir = env::var("OUT_DIR")?;
    let out_dir_path = PathBuf::from(&out_dir);

    let (generated_files, _total_generated_size) = chunk_generator::generate_term_chunks(filtered_terms, &out_dir_path)?;
    index_writer::generate_recognizer_index(generated_files, &out_dir_path, &config.paths.project_root)?;

    Ok(())
}
