use std::{fs, io, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_yaml;
use anyhow::{Result, anyhow};
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PoemFrontMatter {
    title: String,
    summary: String,
    keywords: String,
    emojis: String,
    art_generator_instructions: String,
    memes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    poem_body: Option<String>,
}

fn main() -> Result<()> {
    let poems_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems");

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            // Skip index.md for this specific processing
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Processing: {:?}", path);
            // Fix: Convert &Path to PathBuf and then take a reference
            let path_buf = path.to_path_buf();
            process_poem_file(&path_buf)?;
        }
    }

    Ok(())
}

fn process_poem_file(path: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let parts: Vec<&str> = content.split("---").collect();

    if parts.len() < 3 {
        return Err(anyhow!("Invalid Markdown file format: {:?}", path));
    }

    let front_matter_str = parts[1].trim();
    let poem_body_raw = parts[2..].join("---").trim().to_string(); // Re-join if poem body contains "---"

    let mut front_matter: PoemFrontMatter = serde_yaml::from_str(front_matter_str)?;

    // Move poem body into front matter
    front_matter.poem_body = Some(poem_body_raw);

    // Reconstruct the file content
    let new_content = format!(
        "---\n{}\n---\n",
        serde_yaml::to_string(&front_matter)?
    );

    fs::write(path, new_content)?;

    Ok(())
}
