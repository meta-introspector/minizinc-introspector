use std::{fs, path::PathBuf, collections::HashMap};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use regex::{Regex, Captures};
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
    callback_function: String,
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

    /// Enable debug output, dumping findings in YAML format.
    #[arg(long)]
    debug: bool,
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
    #[serde(skip)] // Don't serialize this field
    pending_meme_description: Option<String>, // Temporary storage for multi-line memes
}

// Define the type for our callback functions
type CallbackFn = Box<dyn Fn(&str, &Captures, &mut FixedFrontMatter) -> Result<()>>;

// Function to create the function registry
fn create_function_registry() -> HashMap<String, CallbackFn> {
    let mut registry: HashMap<String, CallbackFn> = HashMap::new();

    registry.insert(
        "handle_old_meme_regex".to_string(),
        Box::new(|_line, captures, fixed_fm| {
            let description = captures[1].trim().to_string();
            let template = captures[2].trim().to_string();
            fixed_fm.memes.push(Meme {
                description,
                template,
                traits: None,
                nft_id: None,
                lore: None,
                numerology: None,
            });
            Ok(())
        }),
    );

    registry.insert(
        "handle_new_meme_desc_regex".to_string(),
        Box::new(|_line, captures, fixed_fm| {
            fixed_fm.pending_meme_description = Some(captures[1].trim().to_string());
            Ok(())
	}
        ),
    );

    registry.insert(
        "handle_new_meme_template_regex".to_string(),
        Box::new(|_line, captures, fixed_fm| {
            if let Some(description) = fixed_fm.pending_meme_description.take() { // .take() moves the value out, leaving None
                let template = captures[1].trim().to_string();
                fixed_fm.memes.push(Meme {
                    description,
                    template,
                    traits: None,
                    nft_id: None,
                    lore: None,
                    numerology: None,
                });
            } else {
                eprintln!("Warning: Template found without a preceding description for new meme format.");
            }
            Ok(())
        }),
    );

    registry
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    if let Some(file_path) = cli.file {
        println!("Processing single file: {:?}", file_path);
        match process_poem_file(&file_path, cli.max_change_percentage, cli.debug) {
            Ok(_) => println!("Successfully fixed: {:?}\n", file_path),
            Err(e) => eprintln!("Error fixing {:?}: {}\n", file_path, e),
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
                match process_poem_file(&path_buf, cli.max_change_percentage, cli.debug) {
                    Ok(_) => println!("Successfully fixed: {:?}\n", path_buf),
                    Err(e) => eprintln!("Error fixing {:?}: {}\n", path_buf, e),
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
            }
            else {
                in_poem_body_in_fm = false;
                front_matter_str_for_parsing.push_str(line);
                front_matter_str_for_parsing.push('\n');
            }
        }
        else {
            front_matter_str_for_parsing.push_str(line);
            front_matter_str_for_parsing.push('\n');
        }
    }

    Ok((fm_start, fm_end, front_matter_str_for_parsing, extracted_poem_body_from_fm))
}

// New function to parse basic front matter fields
fn parse_front_matter_fields(
    front_matter_str_for_parsing: &str,
    fixed_fm: &mut FixedFrontMatter,
) -> Result<()> {
    let parsed_front_matter: serde_yaml::Value = serde_yaml::from_str(front_matter_str_for_parsing)
        .map_err(|e| anyhow!("Failed to parse front matter YAML: {}", e))?;

    if let Some(title) = parsed_front_matter.get("title").and_then(|v| v.as_str()) {
        fixed_fm.title = Some(title.to_string());
    }
    if let Some(summary) = parsed_front_matter.get("summary").and_then(|v| v.as_str()) {
        fixed_fm.summary = Some(summary.to_string());
    }
    if let Some(keywords) = parsed_front_matter.get("keywords").and_then(|v| v.as_str()) {
        fixed_fm.keywords = Some(keywords.to_string());
    }
    if let Some(emojis) = parsed_front_matter.get("emojis").and_then(|v| v.as_str()) {
        fixed_fm.emojis = Some(emojis.to_string());
    }
    if let Some(art_generator_instructions) = parsed_front_matter.get("art_generator_instructions").and_then(|v| v.as_str()) {
        fixed_fm.art_generator_instructions = Some(art_generator_instructions.to_string());
    }

    Ok(())
}

// New function to process meme entries using the function registry
fn process_memes_with_workflow(
    front_matter_str_for_parsing: &str,
    regex_config: &RegexConfig,
    fixed_fm: &mut FixedFrontMatter,
    function_registry: &HashMap<String, CallbackFn>,
    debug_mode: bool, // Add debug_mode
) -> Result<()> {
    let mut compiled_regexes: HashMap<String, Regex> = HashMap::new();
    for entry in &regex_config.regexes {
        compiled_regexes.insert(entry.name.clone(), Regex::new(&entry.pattern)?);
    }

    for line in front_matter_str_for_parsing.lines() {
        for entry in &regex_config.regexes {
            if let Some(regex) = compiled_regexes.get(&entry.name) {
                if let Some(captures) = regex.captures(line) {
                    if debug_mode {
                        println!("  Matched Regex: {}", entry.name);
                        println!("    Line: {}", line);
                        println!("    Captures: {:?}", captures);
                        println!("    Calling function: {}", entry.callback_function);
                    }
                    if let Some(callback) = function_registry.get(&entry.callback_function) {
                        callback(line, &captures, fixed_fm)?;
                    } else {
                        eprintln!("Warning: Callback function '{}' not found in registry for regex '{}'", entry.callback_function, entry.name);
                    }
                    // Assuming only one regex matches per line for now, break after first match
                    break;
                }
            }
        }
    }
    Ok(())
}

// New function to extract words from text
fn extract_words_from_text(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}


fn process_poem_file(path: &PathBuf, max_change_percentage: Option<f64>, debug_mode: bool) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let original_content_len = content.len();
    let mut lines: Vec<&str> = content.lines().collect();

    let (_fm_start, fm_end, front_matter_str_for_parsing, extracted_poem_body_from_fm) = extract_front_matter(&mut lines, &content)?;
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
        pending_meme_description: None,
    };

    // Call the new function to parse basic front matter fields
    parse_front_matter_fields(&front_matter_str_for_parsing, &mut fixed_fm)?;

    // Load regex patterns from TOML
    let regex_config_str = fs::read_to_string("crates/poem_yaml_fixer/src/regex_patterns.toml")?;
    let regex_config: RegexConfig = toml::from_str(&regex_config_str)?;

    // Create the function registry
    let function_registry = create_function_registry();

    // Call the new function to process meme entries using the workflow
    process_memes_with_workflow(&front_matter_str_for_parsing, &regex_config, &mut fixed_fm, &function_registry, debug_mode)?;

    // After final_poem_body is determined
    let extracted_words = extract_words_from_text(&final_poem_body);

    if debug_mode {
        println!("\n--- Extracted Words ---");
        println!("{:?}", extracted_words);
        println!("-----------------------");
    }

    // Reconstruct the file content
    let mut new_content_parts = Vec::new();
    new_content_parts.push("---".to_string());
    new_content_parts.push(serde_yaml::to_string(&fixed_fm)?);
    new_content_parts.push("---".to_string());

    // Append poem body, prioritizing extracted one
    if let Some(ref pb) = fixed_fm.poem_body {
        new_content_parts.push(pb.clone());
    }
    else {
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

    // Dump findings in YAML if debug mode is enabled
    if debug_mode {
        println!("\n--- Debug Output (Fixed Front Matter) ---");
        println!("{}", serde_yaml::to_string(&fixed_fm)?);
        println!("-----------------------------------------");
    }

    Ok(())
}
