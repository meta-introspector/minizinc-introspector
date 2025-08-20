use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileAnalysis {
    pub path: PathBuf,
    pub word_count: usize,
    pub bag_of_words: HashMap<String, usize>,
    pub last_modified: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectAnalysis {
    pub project_root: PathBuf,
    pub rust_files: Vec<FileAnalysis>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilePairSimilarity {
    pub file1_path: PathBuf,
    pub file2_path: PathBuf,
    pub similarity: f64,
}
