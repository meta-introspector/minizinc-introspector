use std::{fs, path::PathBuf};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;

// Struct for the structured meme (same as in poem_meme_formatter)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Meme {
    description: String,
    template: String,
    // Add any other template parameters you want here
    #[serde(default, skip_serializing_if = "Option::is_none")]
    traits: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    nft_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lore: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    numerology: Option<String>,
}

// A simplified struct to represent the parsed front matter for fixing
// We'll build this up manually
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FixedFrontMatter {
    title: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
    emojis: Option<String>,
    art_generator_instructions: Option<String>,
    memes: Vec<Meme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    poem_body: Option<String>, // This will be extracted if found in FM
}


fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Processing: {:?}", path);
            // Fix: Convert &Path to PathBuf and then pass a reference
            let path_buf = path.to_path_buf(); // Convert &Path to PathBuf
            match process_poem_file(&path_buf) {
                Ok(_) => println!("Successfully fixed: {:?}", path_buf),
                Err(e) => eprintln!("Error fixing {:?}: {}", path_buf, e),
            }
        }
    }

    Ok(())
}

fn process_poem_file(path: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

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
        return Err(anyhow!("Invalid Markdown file format (missing or malformed front matter delimiters).\nContent:\n{}", content));
    }

    let front_matter_lines = &lines[(fm_start + 1) as usize .. fm_end as usize];
    let poem_body_raw_from_file = lines[(fm_end + 1) as usize ..].join("\n");

    let mut fixed_fm = FixedFrontMatter {
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        memes: Vec::new(),
        poem_body: None,
    };

    //let _current_key = String::new();
    let mut in_memes_section = false;
    let mut current_meme_description = String::new();
    let mut current_meme_template = String::new();

    // Corrected Regexes - Define them once at the beginning of the function
    let old_meme_regex = Regex::new(r#"^\s*-\s*"(.*?)"\s*\((.*?)\)$"#)?;
    let new_meme_desc_regex = Regex::new(r#"^\s*-\s*description:\s*"(.*?)"$"#)?;
    let new_meme_template_regex = Regex::new(r#"^\s*template:\s*"(.*?)"$"#)?;


    for line in front_matter_lines.iter() {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with("title:") {
            fixed_fm.title = Some(line.trim_start_matches("title:").trim().to_string());
            in_memes_section = false;
        } else if trimmed_line.starts_with("summary:") {
            fixed_fm.summary = Some(line.trim_start_matches("summary:").trim().to_string());
            in_memes_section = false;
        } else if trimmed_line.starts_with("keywords:") {
            fixed_fm.keywords = Some(line.trim_start_matches("keywords:").trim().to_string());
            in_memes_section = false;
        } else if trimmed_line.starts_with("emojis:") {
            fixed_fm.emojis = Some(line.trim_start_matches("emojis:").trim().to_string());
            in_memes_section = false;
        } else if trimmed_line.starts_with("art_generator_instructions:") {
            fixed_fm.art_generator_instructions = Some(line.trim_start_matches("art_generator_instructions:").trim().to_string());
            in_memes_section = false;
        } else if trimmed_line.starts_with("poem_body:") {
            // Extract poem_body if it's found in the front matter
            fixed_fm.poem_body = Some(line.trim_start_matches("poem_body: |").trim().to_string());
            in_memes_section = false;
        } else if trimmed_line.starts_with("memes:") {
            in_memes_section = true;
            // Reset current meme parsing state
            current_meme_description.clear();
            current_meme_template.clear();
        } else if in_memes_section {
            if new_meme_desc_regex.is_match(line) {
                // Already in new structured format
                if !current_meme_description.is_empty() { // If previous meme was incomplete, add it
                    fixed_fm.memes.push(Meme {
                        description: current_meme_description.clone(),
                        template: current_meme_template.clone(),
                        traits: None, nft_id: None, lore: None, numerology: None
                    });
                }
                current_meme_description = new_meme_desc_regex.captures(line).unwrap()[1].trim().to_string();
                current_meme_template.clear(); // Clear template for next line
            } else if new_meme_template_regex.is_match(line) {
                // Template line for new structured format
                current_meme_template = new_meme_template_regex.captures(line).unwrap()[1].trim().to_string();
                // Add the completed meme
                fixed_fm.memes.push(Meme {
                    description: current_meme_description.clone(),
                    template: current_meme_template.clone(),
                    traits: None, nft_id: None, lore: None, numerology: None
                });
                current_meme_description.clear(); // Clear for next meme
                current_meme_template.clear();
            } else if old_meme_regex.is_match(line) {
                // Old string meme format
                let captures = old_meme_regex.captures(line).unwrap();
                let description = captures[1].trim().to_string();
                let template = captures[2].trim().to_string();
                fixed_fm.memes.push(Meme { description, template, traits: None, nft_id: None, lore: None, numerology: None });
                // Do not set in_memes_section to false here, as there might be more old memes
            } else if line.trim().is_empty() || !line.starts_with(" ") {
                // End of memes section or new top-level key
                in_memes_section = false;
                // If there's a pending meme, add it
                if !current_meme_description.is_empty() {
                    fixed_fm.memes.push(Meme {
                        description: current_meme_description.clone(),
                        template: current_meme_template.clone(),
                        traits: None, nft_id: None, lore: None, numerology: None
                    });
                    current_meme_description.clear();
                    current_meme_template.clear();
                }
            }
        }
    }

    // Handle any remaining pending meme at the end of the file
    if in_memes_section && !current_meme_description.is_empty() {
        fixed_fm.memes.push(Meme {
            description: current_meme_description.clone(),
            template: current_meme_template.clone(),
            traits: None, nft_id: None, lore: None, numerology: None
        });
    }


    // Reconstruct the file content
    let mut new_content_parts = Vec::new();
    new_content_parts.push("---".to_string());
    new_content_parts.push(serde_yaml::to_string(&fixed_fm)?);
    new_content_parts.push("---".to_string());

    // Append poem body, prioritizing extracted one
    if let Some(pb) = fixed_fm.poem_body {
        new_content_parts.push(pb);
    } else {
        new_content_parts.push(poem_body_raw_from_file);
    }


    fs::write(path, new_content_parts.join("\n"))?;

    Ok(())
}
