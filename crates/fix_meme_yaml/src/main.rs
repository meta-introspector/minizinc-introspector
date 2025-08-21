use std::{fs, path::PathBuf};
//use anyhow::{Result, anyhow};
use anyhow::{Result};
use walkdir::WalkDir;
use regex::Regex;

fn main() -> Result<()> {
    let poems_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems");

    let meme_line_regex = Regex::new(r#"^\s*-\s*"(.*?)"\s*\((.*?)\)""#)?;

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Fixing memes in: {:?}", path);
            let content = fs::read_to_string(&path)?;
            let mut new_content_lines = Vec::new();
            let mut in_memes_section = false;
            let mut current_memes = Vec::new();

            for line in content.lines() {
                if line.trim().starts_with("memes:") {
                    in_memes_section = true;
                    new_content_lines.push(line.to_string()); // Keep the "memes:" line
                    continue;
                }

                if in_memes_section {
                    if line.trim().starts_with("-") {
                        if let Some(captures) = meme_line_regex.captures(line) {
                            let description = captures.get(1).map_or("", |m| m.as_str()).trim().to_string();
                            let template = captures.get(2).map_or("", |m| m.as_str()).trim().to_string();
                            current_memes.push(format!("  - description: \"{}\"\n    template: \"{}\"", description, template));
                        } else {
                            // If it's a meme line but doesn't match the expected format, keep it as is for now
                            // This might indicate a meme without a template, or another malformed line
                            current_memes.push(line.to_string());
                        }
                    } else if line.trim().is_empty() || !line.starts_with(" ") {
                        // End of memes section
                        in_memes_section = false;
                        new_content_lines.extend(current_memes.drain(..)); // Add collected memes
                        new_content_lines.push(line.to_string()); // Add the line that ended the section
                    } else {
                        // Line is indented but not a meme item, likely part of a multi-line meme description
                        current_memes.push(line.to_string());
                    }
                } else {
                    new_content_lines.push(line.to_string());
                }
            }
            // Handle case where memes section is at the end of the file
            if in_memes_section {
                new_content_lines.extend(current_memes.drain(..));
            }

            fs::write(&path, new_content_lines.join("\n"))?;
        }
    }

    Ok(())
}
