//use std::fs;
use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn collect_files(root_dir: &PathBuf, extensions: &[&str]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if extensions.contains(&ext_str) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
    }
    Ok(files)
}
