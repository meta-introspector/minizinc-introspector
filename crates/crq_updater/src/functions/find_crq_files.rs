use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const CRQ_FILE_PATTERN: &str = "crq_*.md";

pub fn find_crq_files(repo_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut crq_files = Vec::new();
    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(Result::ok) // Filter out errors
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.starts_with("crq_") && file_name.ends_with(".md") {
                    crq_files.push(path.to_path_buf());
                }
            }
        }
    }
    Ok(crq_files)
}
