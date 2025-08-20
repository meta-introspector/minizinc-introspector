use crate::utils::error::{Result, ZosError};
use crate::cli::IndexUpdateArgs;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Define a struct to store file metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileMetadata {
    path: PathBuf,
    last_modified: u64, // Unix timestamp
    // Add hash later for more robust change detection
}

// Define the overall index metadata structure
#[derive(Serialize, Deserialize, Debug, Default)]
struct IndexMetadata {
    files: HashMap<PathBuf, FileMetadata>,
    last_indexed_time: u64,
}

const METADATA_FILE_NAME: &str = ".zos_index_metadata.json";

pub fn handle_index_update_command(args: IndexUpdateArgs) -> Result<()> {
    println!("Index update command received. Incremental: {}", args.incremental);

    let project_root = crate::utils::paths::resolve_project_root()?;
    let metadata_path = project_root.join(METADATA_FILE_NAME);

    let mut current_metadata = load_metadata(&metadata_path)?;

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    println!("Scanning for changes in: {}", project_root.display());

    let mut new_files = Vec::new();
    let mut modified_files = Vec::new();
    let mut deleted_files = Vec::new();
    let mut unchanged_files = Vec::new();

    let mut files_on_disk: HashMap<PathBuf, FileMetadata> = HashMap::new();

    // Scan relevant file types
    let extensions = ["md", "rs", "cpp", "h", "hpp"];
    for entry in walkdir::WalkDir::new(&project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_path_buf();
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if extensions.contains(&ext) {
                let last_modified = entry.metadata()?.modified()?.duration_since(UNIX_EPOCH).unwrap().as_secs();
                let file_metadata = FileMetadata { path: path.clone(), last_modified };
                files_on_disk.insert(path.clone(), file_metadata.clone());

                if let Some(existing_metadata) = current_metadata.files.get(&path) {
                    if existing_metadata.last_modified < last_modified {
                        modified_files.push(file_metadata);
                    } else {
                        unchanged_files.push(file_metadata);
                    }
                } else {
                    new_files.push(file_metadata);
                }
            }
        }
    }

    // Detect deleted files
    for (path, metadata) in &current_metadata.files {
        if !files_on_disk.contains_key(path) {
            deleted_files.push(metadata.clone());
        }
    }

    println!("Scan complete:");
    println!("  New files: {}", new_files.len());
    println!("  Modified files: {}", modified_files.len());
    println!("  Deleted files: {}", deleted_files.len());
    println!("  Unchanged files: {}", unchanged_files.len());

    // Placeholder for actual indexing logic
    if args.incremental {
        println!("Performing incremental update...");
        // Process new and modified files
        // Update term index
        // Update word embeddings
    } else {
        println!("Performing full re-index...");
        // Process all files
    }

    // Update metadata for next run
    current_metadata.files = files_on_disk;
    current_metadata.last_indexed_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    save_metadata(&metadata_path, &current_metadata)?;

    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("Index update completed in {} seconds.", end_time - start_time);

    Ok(())
}

fn load_metadata(path: &Path) -> Result<IndexMetadata> {
    if path.exists() {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(IndexMetadata::default())
    }
}

fn save_metadata(path: &Path, metadata: &IndexMetadata) -> Result<()> {
    let content = serde_json::to_string_pretty(metadata)?;
    fs::write(path, content)?;
    Ok(())
}
