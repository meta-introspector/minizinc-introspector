use std::{fs, path::PathBuf};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use regex::Regex;
//use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_derive::{Deserialize, Serialize};

// New struct for the structured meme
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Meme {
    description: String,
    template: String,
    // Add any other template parameters you want here
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PoemFrontMatter {
    title: String,
    summary: String,
    keywords: String,
    emojis: String,
    art_generator_instructions: String,
    memes: Vec<Meme>, // Changed to Vec<Meme>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    poem_body: Option<String>,
}

// Temporary struct to parse memes as raw YAML Value first
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TempPoemFrontMatter {
    title: String,
    summary: String,
    keywords: String,
    emojis: String,
    art_generator_instructions: String,
    memes: serde_yaml::Value, // Changed to serde_yaml::Value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    poem_body: Option<String>,
}


fn main() -> Result<()> {
    // Get the current working directory to make paths relative
    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            // Skip index.md for this specific processing
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Processing: {:?}", path);
            // Fix: Convert &Path to PathBuf and then pass a reference
            let path_buf = path.to_path_buf(); // Convert &Path to PathBuf
            process_poem_file(&path_buf)?;
        }
    }

    Ok(())
}

fn process_poem_file(path: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Find the YAML front matter delimiters
    let mut fm_start = -1;
    let mut fm_end = -1;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "---" {
            if fm_start == -1 {
                fm_start = i as isize;
            } else {
                fm_end = i as isize;
                break;
            }
        }
    }

    if fm_start != 0 || fm_end == -1 {
        return Err(anyhow!("Invalid Markdown file format (missing or malformed front matter delimiters): {:?}", path));
    }

    let front_matter_str = &lines[(fm_start + 1) as usize .. fm_end as usize].join("\n");
    let poem_body_lines = &lines[(fm_end + 1) as usize ..];

    let mut parsed_front_matter: serde_yaml::Value = serde_yaml::from_str(front_matter_str)?;

    let mut structured_memes: Vec<Meme> = Vec::new();
    let meme_regex = Regex::new(r"^(.*?)\s*\((.*?)\)$")?;

    if let Some(memes_value) = parsed_front_matter.get("memes") {
        if let Some(meme_array) = memes_value.as_sequence() {
            for meme_value in meme_array {
                if let Some(meme_str) = meme_value.as_str() {
                    if let Some(captures) = meme_regex.captures(meme_str) {
                        let description = captures.get(1).map_or("", |m| m.as_str()).trim().to_string();
                        let template = captures.get(2).map_or("", |m| m.as_str()).trim().to_string();
                        structured_memes.push(Meme { description, template });
                    } else {
                        // If it's a string but doesn't match the (template) format, use it as description and "default" template
                        structured_memes.push(Meme {
                            description: meme_str.trim().to_string(),
                            template: "default".to_string(),
                        });
                    }
                } else if let Some(_meme_map) = meme_value.as_mapping() {
                    // If it's already a map (new format), try to deserialize it directly
		    // fixme: use meme_map
                    let meme: Meme = serde_yaml::from_value(meme_value.clone())?;
                    structured_memes.push(meme);
                }
            }
        }
    }

    // Update the memes field in the parsed_front_matter
    parsed_front_matter["memes"] = serde_yaml::to_value(&structured_memes)?;
    parsed_front_matter["poem_body"] = serde_yaml::to_value(poem_body_lines.join("\n"))?;


    // Reconstruct the file content
    let new_content = format!(
        "---\n{}\n---\n{}",
        serde_yaml::to_string(&parsed_front_matter)?,
        poem_body_lines.join("\n")
    );

    fs::write(path, new_content)?;

    Ok(())
}
