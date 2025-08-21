//use std::collections::HashMap;
use anyhow::{Result, anyhow};
use regex::Regex;
use serde_yaml;

use crate::models::{Meme, PoemDocument};

pub fn parse_poem_document(content: &str) -> Result<PoemDocument> {
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
        return Err(anyhow!("Invalid Markdown file format (missing or malformed front matter delimiters)."));
    }

    let front_matter_lines_slice = &lines[(fm_start + 1) as usize .. fm_end as usize];
    let poem_body_lines_vec: Vec<String> = lines[(fm_end + 1) as usize ..].iter().map(|s| s.to_string()).collect();

    // Attempt to parse the front matter into a serde_yaml::Value
    let mut parsed_front_matter: serde_yaml::Value = serde_yaml::from_str(&front_matter_lines_slice.join("\n"))
        .map_err(|e| anyhow!("Failed to parse front matter YAML: {}", e))?;

    let mut extracted_poem_body = String::new();

    // Check if 'poem_body' exists within the parsed front matter
    if let Some(pb_value) = parsed_front_matter.get("poem_body") {
        if let Some(pb_str) = pb_value.as_str() {
            extracted_poem_body = pb_str.to_string();
            // Remove 'poem_body' from the front matter so it's not serialized back into it
            if let Some(map) = parsed_front_matter.as_mapping_mut() {
                map.remove(&serde_yaml::Value::String("poem_body".to_string()));
            }
        }
    }

    // If poem_body was extracted from front matter, it overrides the content after ---
    let final_poem_body = if !extracted_poem_body.is_empty() {
        extracted_poem_body
    } else {
        poem_body_lines_vec.join("\n")
    };


    let mut title = String::new();
    let mut summary = String::new();
    let mut keywords = String::new();
    let mut emojis = String::new();
    let mut art_generator_instructions = String::new();
    let mut memes: Vec<Meme> = Vec::new();

    // Extract other fields from parsed_front_matter
    if let Some(t) = parsed_front_matter.get("title").and_then(|v| v.as_str()) { title = t.to_string(); }
    if let Some(s) = parsed_front_matter.get("summary").and_then(|v| v.as_str()) { summary = s.to_string(); }
    if let Some(k) = parsed_front_matter.get("keywords").and_then(|v| v.as_str()) { keywords = k.to_string(); }
    if let Some(e) = parsed_front_matter.get("emojis").and_then(|v| v.as_str()) { emojis = e.to_string(); }
    if let Some(a) = parsed_front_matter.get("art_generator_instructions").and_then(|v| v.as_str()) { art_generator_instructions = a.to_string(); }

    // Process memes
    let meme_regex = Regex::new(r#"^\s*-\s*description:\s*\"(.*?)\"\s*template:\s*\"(.*?)\"$"#)?;
    let old_meme_regex = Regex::new(r#"^\s*-\s*\"(.*?)\"\s*\((.*?)\)$"#)?;

    if let Some(memes_value) = parsed_front_matter.get("memes") {
        if let Some(meme_array) = memes_value.as_sequence() {
            for meme_value in meme_array {
                if let Some(meme_str) = meme_value.as_str() {
                    if let Some(captures) = meme_regex.captures(meme_str) {
                        let description = captures[1].trim().to_string();
                        let template = captures[2].trim().to_string();
                        memes.push(Meme { description, template, traits: None, nft_id: None, lore: None, numerology: None });
                    } else if let Some(captures) = old_meme_regex.captures(meme_str) {
                        let description = captures[1].trim().to_string();
                        let template = captures[2].trim().to_string();
                        memes.push(Meme { description, template, traits: None, nft_id: None, lore: None, numerology: None });
                    } else {
                        // Fallback for unparseable meme strings
                        memes.push(Meme {
                            description: meme_str.trim().to_string(),
                            template: "default".to_string(),
                            traits: None, nft_id: None, lore: None, numerology: None
                        });
                    }
                } else if let Some(_meme_map) = meme_value.as_mapping() {
                    // If it's already a map (new format), try to deserialize it directly
                    let meme: Meme = serde_yaml::from_value(meme_value.clone())
                        .map_err(|e| anyhow!("Failed to deserialize meme map: {}", e))?;
                    memes.push(meme);
                }
            }
        }
    }


    Ok(PoemDocument {
        title,
        summary,
        keywords,
        emojis,
        art_generator_instructions,
        memes,
        poem_body: final_poem_body,
    })
}
