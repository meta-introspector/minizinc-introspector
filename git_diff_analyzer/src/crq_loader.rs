use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;

use crate::text_processor::{clean_and_tokenize, filter_stop_words, generate_bag_of_words};

pub struct CrqDocument {
    pub path: PathBuf,
    pub content: String,
    pub bag_of_words: HashMap<String, usize>,
}

pub fn load_crq_documents<P: AsRef<Path>>(crq_dir: P) -> Result<Vec<CrqDocument>> {
    let mut crq_docs = Vec::new();

    for entry in WalkDir::new(&crq_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(path)?;
            
            // Simple attempt to strip front matter (lines between ---)
            let processed_content = if content.starts_with("---") {
                let mut lines = content.lines();
                lines.next(); // Skip first ---
                let mut stripped_content = String::new();
                let mut in_front_matter = true;
                for line in lines {
                    if in_front_matter && line == "---" {
                        in_front_matter = false;
                        continue;
                    }
                    if !in_front_matter {
                        stripped_content.push_str(line);
                        stripped_content.push_str("\n");
                    }
                }
                stripped_content
            } else {
                content
            };

            let cleaned_words = clean_and_tokenize(&processed_content);
            let filtered_words = filter_stop_words(cleaned_words);
            let bag_of_words = generate_bag_of_words(filtered_words);

            crq_docs.push(CrqDocument {
                path: path.to_path_buf(),
                content: processed_content,
                bag_of_words,
            });
        }
    }
    Ok(crq_docs)
}
