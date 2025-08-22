use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
use regex::Regex;
use once_cell::sync::Lazy;


// Import callback functions for simple fields
use crate::functions::callbacks::handle_art_generator_instructions_regex::handle_art_generator_instructions_regex;
use crate::functions::callbacks::handle_emojis_regex::handle_emojis_regex;
use crate::functions::callbacks::handle_keywords_regex::handle_keywords_regex;
use crate::functions::callbacks::handle_new_document::handle_new_document;
use crate::functions::callbacks::handle_summary_regex::handle_summary_regex;
use crate::functions::callbacks::handle_title_regex::handle_title_regex;

// Define Lazy Regex instances for simple patterns
static ART_GENERATOR_INSTRUCTIONS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"art_generator_instructions:\s*(.*)").unwrap());
static EMOJIS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"emojis:\s*(.*)").unwrap());
static KEYWORDS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"keywords:\s*(.*)").unwrap());
static NEW_DOCUMENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"---").unwrap());
static SUMMARY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"summary:\s*(.*)").unwrap());
static TITLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"title:\s*(.*)").unwrap());

pub fn manual_parse_poem_file(content: &str, fixed_fm: &mut FixedFrontMatter) -> Result<()> {
    let mut in_front_matter = false;

    for line in content.lines() {
        if NEW_DOCUMENT_REGEX.is_match(line) {
            if !in_front_matter {
                in_front_matter = true;
                handle_new_document(line, vec![], fixed_fm)?;
            } else {
                // End of front matter
                in_front_matter = false;
            }
            continue;
        }

        if in_front_matter {
            // Attempt to match and call callbacks for front matter fields
            if TITLE_REGEX.is_match(line) {
                let captures = TITLE_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_title_regex(line, captures, fixed_fm)?;
            } else if SUMMARY_REGEX.is_match(line) {
                let captures = SUMMARY_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_summary_regex(line, captures, fixed_fm)?;
            } else if KEYWORDS_REGEX.is_match(line) {
                let captures = KEYWORDS_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_keywords_regex(line, captures, fixed_fm)?;
            } else if EMOJIS_REGEX.is_match(line) {
                let captures = EMOJIS_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_emojis_regex(line, captures, fixed_fm)?;
            } else if ART_GENERATOR_INSTRUCTIONS_REGEX.is_match(line) {
                let captures = ART_GENERATOR_INSTRUCTIONS_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_art_generator_instructions_regex(line, captures, fixed_fm)?;
            }
            // No other complex parsing for memes or poem body in this simplified manual parser
        }
    }

    Ok(())
}