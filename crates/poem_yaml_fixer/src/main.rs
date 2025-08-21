use std::{fs, path::PathBuf};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use toml;
use clap::Parser;

// Define structs for TOML deserialization
#[derive(Debug, Deserialize)]
struct RegexConfig {
    regexes: Vec<RegexEntry>,
}

#[derive(Debug, Deserialize)]
struct RegexEntry {
    name: String,
    pattern: String,
}

// Add Cli struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional path to a single poem file to process. If not provided, processes all .md files in docs/poems/.
    #[arg(short, long, value_name = "FILE_PATH")]
    file: Option<PathBuf>,

    /// Maximum allowed percentage of content reduction. Aborts if reduction exceeds this. Defaults to 1.0.
    #[arg(long, value_name = "PERCENTAGE")]
    max_change_percentage: Option<f64>,
}

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
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    if let Some(file_path) = cli.file {
        println!("Processing single file: {:?}", file_path);
        match process_poem_file(&file_path, cli.max_change_percentage) {
            Ok(_) => println!("Successfully fixed: {:?}", file_path),
            Err(e) => eprintln!("Error fixing {:?}: {}", file_path, e),
        }
    } else {
        for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if path.file_name().map_or(false, |name| name == "index.md") {
                    continue;
                }

                println!("Processing: {:?}", path);
                let path_buf = path.to_path_buf();
                match process_poem_file(&path_buf, cli.max_change_percentage) {
                    Ok(_) => println!("Successfully fixed: {:?}", path_buf),
                    Err(e) => eprintln!("Error fixing {:?}: {}", path_buf, e),
                }
            }
        }
    }

    Ok(())
}

// New function to extract front matter
fn extract_front_matter(lines: &mut Vec<&str>, content: &str) -> Result<(isize, isize, String, String)> {
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

    let front_matter_lines_slice = &lines[(fm_start + 1) as usize .. fm_end as usize];
    let mut front_matter_str_for_parsing = String::new();
    let mut extracted_poem_body_from_fm = String::new();
    let mut in_poem_body_in_fm = false;

    // Manually process front matter lines to extract poem_body if present
    for line in front_matter_lines_slice.iter() {
        if line.trim().starts_with("poem_body: |") {
            in_poem_body_in_fm = true;
        } else if in_poem_body_in_fm {
            if line.starts_with(" ") {
                extracted_poem_body_from_fm.push_str(line.trim_start());
                extracted_poem_body_from_fm.push('\n');
            } else {
                in_poem_body_in_fm = false;
                front_matter_str_for_parsing.push_str(line);
                front_matter_str_for_parsing.push('\n');
            }
        } else {
            front_matter_str_for_parsing.push_str(line);
            front_matter_str_for_parsing.push('\n');
        }
    }

    Ok((fm_start, fm_end, front_matter_str_for_parsing, extracted_poem_body_from_fm))
}


fn process_poem_file(path: &PathBuf, max_change_percentage: Option<f64>) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let original_content_len = content.len();
    let mut lines: Vec<&str> = content.lines().collect();

    let (fm_start, fm_end, front_matter_str_for_parsing, extracted_poem_body_from_fm) = extract_front_matter(&mut lines, &content)?;
    let poem_body_raw_from_file = lines[(fm_end + 1) as usize ..].join("\n");

    let final_poem_body = if !extracted_poem_body_from_fm.is_empty() {
        extracted_poem_body_from_fm
    } else {
        poem_body_raw_from_file
    };

    let mut fixed_fm = FixedFrontMatter {
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        memes: Vec::new(),
        poem_body: None,
    };

    let mut current_key = String::new();
    let mut in_memes_section = false;
    let mut current_meme_description = String::new();
    let mut current_meme_template = String::new();

    // Load regex patterns from TOML
    let regex_config_str = fs::read_to_string("crates/poem_yaml_fixer/src/regex_patterns.toml")?;
    let regex_config: RegexConfig = toml::from_str(&regex_config_str)?;

    let old_meme_regex_pattern = regex_config.regexes.iter()
        .find(|r| r.name == "old_meme_regex")
        .ok_or_else(|| anyhow!("old_meme_regex not found in config"))?.pattern.clone();
    let new_meme_desc_regex_pattern = regex_config.regexes.iter()
        .find(|r| r.name == "new_meme_desc_regex")
        .ok_or_else(|| anyhow!("new_meme_desc_regex not found in config"))?.pattern.clone();
    let new_meme_template_regex_pattern = regex_config.regexes.iter()
        .find(|r| r.name == "new_meme_template_regex")
        .ok_or_else(|| anyhow!("new_meme_template_regex not found in config"))?.pattern.clone();

    let old_meme_regex = Regex::new(&old_meme_regex_pattern)?;
    let new_meme_desc_regex = Regex::new(&new_meme_desc_regex_pattern)?;
    let new_meme_template_regex = Regex::new(&new_meme_template_regex_pattern)?;


    // Attempt to parse the (potentially modified) front matter into a serde_yaml::Value
    let mut parsed_front_matter: serde_yaml::Value = serde_yaml::from_str(&front_matter_str_for_parsing)
        .map_err(|e| anyhow!("Failed to parse front matter YAML after poem_body extraction: {}", e))?;


    // This loop needs to be refactored into its own function
    // for line in front_matter_lines_slice.iter() { ... }

    // Reconstruct the file content
    let mut new_content_parts = Vec::new();
    new_content_parts.push("---".to_string());
    new_content_parts.push(serde_yaml::to_string(&fixed_fm)?);
    new_content_parts.push("---".to_string());

    // Append poem body, prioritizing extracted one
    if let Some(pb) = fixed_fm.poem_body {
        new_content_parts.push(pb);
    } else {
        new_content_parts.push(final_poem_body);
    }

    let new_content = new_content_parts.join("\n");
    let new_content_len = new_content.len();

    // Calculate change percentage and apply abort logic
    let change_percentage = (original_content_len as f64 - new_content_len as f64).abs() / original_content_len as f64 * 100.0;
    let effective_max_change = max_change_percentage.unwrap_or(1.0);

    if change_percentage > effective_max_change {
        return Err(anyhow!(
            "Aborting: Content change exceeds allowed limit. Original size: {}, New size: {}, Change: {:.2}%. Max allowed: {:.2}%",
            original_content_len,
            new_content_len,
            change_percentage,
            effective_max_change
        ));
    }

    fs::write(path, new_content)?;

    Ok(())
}
