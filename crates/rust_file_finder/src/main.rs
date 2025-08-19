use walkdir::WalkDir;
use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

// Function to find the Cargo.toml for a given file path
fn find_cargo_toml_for_file(file_path: &Path) -> Option<PathBuf> {
    let mut current_dir = file_path.parent()?.to_path_buf();
    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        if cargo_toml_path.exists() {
            return Some(cargo_toml_path);
        }
        if !current_dir.pop() { // Move up one directory
            break; // Reached root, no Cargo.toml found
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");
    eprintln!("Starting search for Rust files in: {:?}", search_root);

    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        eprintln!("Processing entry: {:?}", path);
        if path.is_file() {
            if path.extension().map_or(false, |ext| ext == "rs") {
                eprintln!("\nFound Rust file: {:?}", path);
                eprintln!("Searching for Cargo.toml for file: {:?}", path);

                if let Some(cargo_toml_path) = find_cargo_toml_for_file(path) {
                    eprintln!("Found Cargo.toml at: {:?}", cargo_toml_path);
                    match fs::read_to_string(&cargo_toml_path) {
                        Ok(content) => {
                            match toml::from_str::<CargoToml>(&content) {
                                Ok(cargo_info) => {
                                    eprintln!("  Belongs to package: {} (version {})", cargo_info.package.name, cargo_info.package.version);
                                    eprintln!("  Cargo.toml path: {:?}", cargo_toml_path);
                                },
                                Err(e) => {
                                    eprintln!("  Error parsing Cargo.toml at {:?}: {}", cargo_toml_path, e);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("  Error reading Cargo.toml at {:?}: {}", cargo_toml_path, e);
                        }
                    }
                } else {
                    eprintln!("  Does not appear to belong to a Rust package (no Cargo.toml found).");
                }
            } else {
                eprintln!("Skipping non-Rust file: {:?}", path);
            }
        } else {
            eprintln!("Skipping non-file entry: {:?}", path);
        }
    }
    eprintln!("Search complete.");

    Ok(())
}
