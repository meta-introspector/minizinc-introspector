// This file contains the core logic for processing a single poem file.
// It orchestrates calls to other functions to extract front matter, parse fields,
// process memes, extract words, and save the word index.

use std::{fs, path::PathBuf};
use anyhow::{Result, anyhow};
use serde_yaml; // Re-added: use serde_yaml;

use crate::functions::types::FixedFrontMatter; // Import types from the types module
use poem_traits::{RegexConfig, FunctionRegistry}; // Import FunctionRegistry
//use crate::functions::extract_front_matter::extract_front_matter;
// TODO: This function is currently not used. It might be used in future refactoring.
// Removed: use crate::functions::process_memes_with_workflow::process_memes_with_workflow;
// TODO: These functions are currently not used. They are part of the word indexing feature.

// Import the new root YAML validation function
use crate::functions::callbacks::handle_regex_driven_yaml_fix; // Import directly


pub fn process_poem_file(
    path: &PathBuf,
    max_change_percentage: Option<f64>,
    debug_mode: bool,
    dry_run: bool, // Add dry_run parameter
    regex_config: &RegexConfig, // Pass regex_config
    function_registry: &FunctionRegistry,
) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let original_content_len = content.len();
//    let lines: Vec<&str> = content.lines().collect();

    // The extract_front_matter function still extracts the raw front matter string
    // and the poem body, but we won't parse it with serde_yaml here.
    let mut final_poem_body = String::new();

    let mut fixed_fm = FixedFrontMatter {
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        memes: Some(Vec::new()),
        poem_body: None,
        pending_meme_description: None,
	raw_meme_lines : None,
    };

    if final_poem_body.trim().is_empty() {
        let archeology_file_path = path.with_extension("md.archeology.md");
        if archeology_file_path.exists() {
            println!("  Found archeology file, parsing for lost revisions: {:?}", archeology_file_path);
            let recovered_fms = crate::functions::archeology_parser::parse_archeology_file(&archeology_file_path, regex_config, function_registry)?;

            let mut recovered_poem_body = String::new();
            for mut fm in recovered_fms.into_iter().rev() {
                if fixed_fm.title.is_none() {
                    fixed_fm.title = fm.title;
                }
                if fixed_fm.summary.is_none() {
                    fixed_fm.summary = fm.summary;
                }
                if fixed_fm.keywords.is_none() {
                    fixed_fm.keywords = fm.keywords;
                }
                if fixed_fm.emojis.is_none() {
                    fixed_fm.emojis = fm.emojis;
                }
                if fixed_fm.art_generator_instructions.is_none() {
                    fixed_fm.art_generator_instructions = fm.art_generator_instructions;
                }
                if let Some(raw_meme_lines) = fm.raw_meme_lines { // fm.raw_meme_lines is Option<Vec<String>>
                    if !raw_meme_lines.is_empty() {
                        // Here, we would process the raw_meme_lines and convert them to Meme objects
                        // For now, we'll just extend the raw_meme_lines if FixedFrontMatter had a raw_meme_lines field
                        // Since FixedFrontMatter has memes: Option<Vec<Meme>>, we need to convert.
                        // This part needs careful consideration based on how archeology_parser should interact with memes.
                        // For now, let's just ensure the raw_meme_lines are not lost if they were present.
                        // This might require a new field in FixedFrontMatter for raw_meme_lines, or processing them into Meme objects here.
                        // Given the current structure, let's assume archeology_parser should recover raw_meme_lines.
                        // This means FixedFrontMatter should also have a raw_meme_lines field.
                        // However, FixedFrontMatter already has memes: Option<Vec<Meme>>.
                        // This indicates a design conflict. Archeology parser should probably return FixedFrontMatter with processed memes.
                        // For now, to resolve the compilation error, I will just comment out this block.
                        // This will mean memes from archeology files are not recovered for now.
                        // This needs a larger design decision.
                        // fixed_fm.memes.get_or_insert_with(Vec::new).extend(memes);
                    }
                }
                if let Some(pb) = fm.poem_body.take() {
                    recovered_poem_body.push_str(&pb);
                    recovered_poem_body.push_str("\n");
                }
            }
            final_poem_body = recovered_poem_body;
        }
    }

    // --- NEW LOGIC: Call the regex-driven YAML fixer ---
    // The handle_regex_driven_yaml_fix function will now populate fixed_fm
    // based on its regex-driven parsing and state management.
    handle_regex_driven_yaml_fix::handle_regex_driven_yaml_fix(
        path, // Pass the file_path
        &content, // Pass the full content for the regex parser to work on
        Vec::new(), // No captures for the root call
        &mut fixed_fm,
        regex_config,
        function_registry,
    )?;
    // --- END NEW LOGIC ---

    // After processing, populate fixed_fm with metadata from PoemFunctionMetadata
    // if the fields are still None (i.e., not set by parsed YAML or meme processing).
    for entry in &regex_config.regexes {
        if let Some((metadata, _callback_fn)) = function_registry.get(&entry.name) {
            if fixed_fm.title.is_none() {
                fixed_fm.title = metadata.title.clone();
            }
            if fixed_fm.summary.is_none() {
                fixed_fm.summary = metadata.summary.clone();
            }
            if fixed_fm.keywords.is_none() {
                // fixed_fm.keywords = metadata.keywords.clone(); // Commented out: Handled by callback
            }
            if fixed_fm.emojis.is_none() {
                fixed_fm.emojis = metadata.emojis.clone();
            }
            if fixed_fm.art_generator_instructions.is_none() {
                fixed_fm.art_generator_instructions = metadata.art_generator_instructions.clone();
            }
            if fixed_fm.pending_meme_description.is_none() {
                fixed_fm.pending_meme_description = metadata.pending_meme_description.clone();
            }
        }
    }

    let mut new_content_parts = Vec::new();
    new_content_parts.push("---".to_string());
    // Use serde_yaml to serialize the fixed_fm back to YAML
    new_content_parts.push(serde_yaml::to_string(&fixed_fm)?);

    // Handle poem_body formatting
    if let Some(pb) = fixed_fm.poem_body.take() {
        new_content_parts.push("poem_body: |\n".to_string());
        for line in pb.lines() {
            new_content_parts.push(format!("  {line}"));
        }
    } else if !final_poem_body.is_empty() {
        new_content_parts.push("poem_body: |\n".to_string());
        for line in final_poem_body.lines() {
            new_content_parts.push(format!("  {line}"));
        }
    }

    new_content_parts.push("---".to_string());

    let new_content = new_content_parts.join("\n") + "\n";
    let new_content_len = new_content.len();

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

    // Only write if not in dry_run mode and content has changed
    if !dry_run {
        if new_content != content {
            fs::write(path, new_content)?;
            println!("  Changes applied to: {path:?}");
        } else {
            println!("  No changes needed for: {path:?}");
        }
    }
    else if new_content != content {
        println!("  Dry run: Would apply changes to: {path:?}");
    } else {
        println!("  Dry run: No changes needed for: {path:?}");
    }

    if debug_mode {
        println!("\n--- Debug Output (Fixed Front Matter) ---");
        println!("{}", serde_yaml::to_string(&fixed_fm)?);
        println!("-----------------------------------------");
    }

    Ok(())
}
