use std::{fs,
	  //io,
	  path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_yaml;
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use regex::Regex;

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
    let poems_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems");

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            // Skip index.md for this specific processing
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Processing: {:?}", path);
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
    let poem_body_raw = parts[2..].join("---").trim().to_string();

    // Deserialize into TempPoemFrontMatter first
    let temp_front_matter: TempPoemFrontMatter = serde_yaml::from_str(front_matter_str)?;

    let meme_regex = Regex::new(r"^(.*?)\s*\((.*)\)$")?;
    let mut structured_memes: Vec<Meme> = Vec::new();

    // Process the raw serde_yaml::Value for memes
    if let Some(meme_array) = temp_front_matter.memes.as_sequence() {
        for meme_value in meme_array {
            if let Some(meme_str) = meme_value.as_str() {
                // Attempt to parse the old string format
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
                let meme: Meme = serde_yaml::from_value(meme_value.clone())?;
                structured_memes.push(meme);
            } else {
                // Handle other unexpected types in the memes array
                structured_memes.push(Meme {
                    description: format!("Unexpected meme value type: {:?}", meme_value),
                    template: "error".to_string(),
                });
            }
        }
    } else if let Some(meme_str) = temp_front_matter.memes.as_str() {
        // Handle case where 'memes' is a single string, not an array
        if let Some(captures) = meme_regex.captures(meme_str) {
            let description = captures.get(1).map_or("", |m| m.as_str()).trim().to_string();
            let template = captures.get(2).map_or("", |m| m.as_str()).trim().to_string();
            structured_memes.push(Meme { description, template });
        } else {
            structured_memes.push(Meme {
                description: meme_str.trim().to_string(),
                template: "default".to_string(),
            });
        }
    } else {
        // Handle cases where 'memes' field is missing or malformed
        structured_memes.push(Meme {
            description: format!("Malformed or missing memes field: {:?}", temp_front_matter.memes),
            template: "error".to_string(),
        });
    }

    // Construct the final PoemFrontMatter
    let final_front_matter = PoemFrontMatter {
        title: temp_front_matter.title,
        summary: temp_front_matter.summary,
        keywords: temp_front_matter.keywords,
        emojis: temp_front_matter.emojis,
        art_generator_instructions: temp_front_matter.art_generator_instructions,
        memes: structured_memes,
        poem_body: Some(poem_body_raw),
    };

    // Reconstruct the file content
    let new_content = format!(
        "---\n{}\n---\n",
        serde_yaml::to_string(&final_front_matter)?
    );

    fs::write(path, new_content)?;

    Ok(())
}
