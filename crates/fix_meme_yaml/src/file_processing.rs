use std::{fs, path::PathBuf, collections::HashMap};
use anyhow::{Result, anyhow};
use regex::Regex;
use serde_yaml;

//use crate::models::{Meme, PoemFrontMatter, TempPoemFrontMatter};
use crate::models::{Meme};
//use crate::patching;

pub fn process_poem_file(path: &PathBuf, patch_function_name: &Option<String>, patch_functions: &HashMap<String, fn(&str) -> String>) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect(); // Use String for mutability

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
        return Err(anyhow!("Invalid Markdown file format (missing or malformed front matter delimiters): {:?}\nContent:\n{}", path, content));
    }

    let mut front_matter_lines_vec: Vec<String> = lines[(fm_start + 1) as usize .. fm_end as usize].to_vec();
    let poem_body_lines = &lines[(fm_end + 1) as usize ..];

    let mut parsed_front_matter: serde_yaml::Value;
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 5;

    loop {
        match serde_yaml::from_str(&front_matter_lines_vec.join("\n")) {
            Ok(value) => {
                parsed_front_matter = value;
                break;
            },
            Err(e) => {
                attempts += 1;
                if attempts > MAX_ATTEMPTS {
                    return Err(anyhow!("Failed to parse front matter YAML after {} attempts for {:?}: {}", MAX_ATTEMPTS, path, e));
                }

                eprintln!("YAML parsing error in {path:?} (attempt {attempts}): {e}");
                let error_line_num = e.location().map(|loc| loc.line()).unwrap_or(0);

                if error_line_num > 0 && error_line_num <= front_matter_lines_vec.len() {
                    let problematic_line_index = error_line_num - 1;
                    let original_problematic_line = front_matter_lines_vec[problematic_line_index].clone();
                    eprintln!("Problematic line (original): \"{original_problematic_line}\"");

                    if let Some(patch_fn_name) = patch_function_name {
                        if let Some(patch_fn) = patch_functions.get(patch_fn_name) {
                            let patched_line = patch_fn(&original_problematic_line);
                            eprintln!("Attempting patch with '{patch_fn_name}': \"{patched_line}\"");
                            front_matter_lines_vec[problematic_line_index] = patched_line;
                        } else {
                            return Err(anyhow!("Patch function '{}' not found.", patch_fn_name));
                        }
                    } else {
                        return Err(anyhow!("YAML parsing error. No patch function provided. Error: {}", e));
                    }
                } else {
                    return Err(anyhow!("YAML parsing error: Could not pinpoint problematic line for patching in {:?}: {}", path, e));
                }
            }
        }
    }

    let mut structured_memes: Vec<Meme> = Vec::new();
    let meme_regex = Regex::new(r#"^\s*-\s*"(.*?)"\s*\((.*?)\)$"#).unwrap(); // Use raw string literal for regex, and .unwrap() for testing

    if let Some(memes_value) = parsed_front_matter.get("memes") {
        if let Some(meme_array) = memes_value.as_sequence() {
            for meme_value in meme_array {
                if let Some(meme_str) = meme_value.as_str() {
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
                } else if let Some(_meme_map) = meme_value.as_mapping() {
                    let meme: Meme = serde_yaml::from_value(meme_value.clone())
                        .map_err(|e| anyhow!("Failed to deserialize meme map for {:?}: {}", path, e))?;
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
