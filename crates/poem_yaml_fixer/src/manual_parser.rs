use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
use regex::Regex;
use once_cell::sync::Lazy;
use poem_traits::PoemFrontMatterTrait;

// Import all callback functions
use crate::functions::callbacks::handle_art_generator_instructions_regex::handle_art_generator_instructions_regex;
use crate::functions::callbacks::handle_comma_separated_keywords::handle_comma_separated_keywords;
use crate::functions::callbacks::handle_emojis_regex::handle_emojis_regex;
use crate::functions::callbacks::handle_keywords_regex::handle_keywords_regex;
use crate::functions::callbacks::handle_malformed_meme_list_item::handle_malformed_meme_list_item;
use crate::functions::callbacks::handle_new_document::handle_new_document;
use crate::functions::callbacks::handle_new_meme_desc_regex::handle_new_meme_desc_regex;
use crate::functions::callbacks::handle_new_meme_template_regex::handle_new_meme_template_regex;
use crate::functions::callbacks::handle_old_meme_regex::handle_old_meme_regex;
use crate::functions::callbacks::handle_poem_body_start_regex::handle_poem_body_start_regex;
// handle_regex_driven_yaml_fix is an orchestrator, not a simple line-matcher
// handle_root_yaml_validation is an orchestrator, not a simple line-matcher
use crate::functions::callbacks::handle_summary_regex::handle_summary_regex;
use crate::functions::callbacks::handle_title_regex::handle_title_regex;
use crate::functions::callbacks::handle_unquoted_colon_in_description::handle_unquoted_colon_in_description;

// Define Lazy Regex instances for each pattern
static ART_GENERATOR_INSTRUCTIONS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"art_generator_instructions:\s*(.*)").unwrap());
static COMMA_SEPARATED_KEYWORDS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"keywords:\s*(.*)").unwrap());
static EMOJIS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"emojis:\s*(.*)").unwrap());
static KEYWORDS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"keywords:\s*(.*)").unwrap());
static MALFORMED_MEME_LIST_ITEM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^- description:\s*(.*)").unwrap());
static NEW_DOCUMENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"---").unwrap());
static NEW_MEME_DESC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"desc:\s*(.*)").unwrap());
static NEW_MEME_TEMPLATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"template:\s*(.*)").unwrap());
static OLD_MEME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"description:\s*(.*)\s*template:\s*(.*)").unwrap());
static POEM_BODY_START_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"poem_body:\s*|\\").unwrap());
static SUMMARY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"summary:\s*(.*)").unwrap());
static TITLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"title:\s*(.*)").unwrap());
static UNQUOTED_COLON_IN_DESCRIPTION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^- description:\s*(.*)").unwrap());

pub fn manual_parse_poem_file(content: &str, fixed_fm: &mut FixedFrontMatter) -> Result<()> {
    let mut in_front_matter = false;
    let mut front_matter_content = String::new();
    let mut poem_body_content = String::new();
    let mut in_poem_body = false;

    for line in content.lines() {
        if NEW_DOCUMENT_REGEX.is_match(line) {
            if !in_front_matter {
                in_front_matter = true;
                handle_new_document(line, vec![], fixed_fm)?;
            } else {
                // End of front matter
                in_front_matter = false;
                in_poem_body = true; // Start collecting poem body after second ---
            }
            continue;
        }

        if in_front_matter {
            front_matter_content.push_str(line);
            front_matter_content.push('\n');

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
            } else if COMMA_SEPARATED_KEYWORDS_REGEX.is_match(line) { // Added this
                let captures = COMMA_SEPARATED_KEYWORDS_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_comma_separated_keywords(line, captures, fixed_fm)?;
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
            } else if NEW_MEME_DESC_REGEX.is_match(line) {
                let captures = NEW_MEME_DESC_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_new_meme_desc_regex(line, captures, fixed_fm)?;
            } else if NEW_MEME_TEMPLATE_REGEX.is_match(line) {
                let captures = NEW_MEME_TEMPLATE_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_new_meme_template_regex(line, captures, fixed_fm)?;
            } else if OLD_MEME_REGEX.is_match(line) {
                let captures = OLD_MEME_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_old_meme_regex(line, captures, fixed_fm)?;
            } else if MALFORMED_MEME_LIST_ITEM_REGEX.is_match(line) { // Added this
                let captures = MALFORMED_MEME_LIST_ITEM_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_malformed_meme_list_item(line, captures, fixed_fm)?;
            } else if POEM_BODY_START_REGEX.is_match(line) {
                let captures = POEM_BODY_START_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_poem_body_start_regex(line, captures, fixed_fm)?;
            } else if UNQUOTED_COLON_IN_DESCRIPTION_REGEX.is_match(line) {
                let captures = UNQUOTED_COLON_IN_DESCRIPTION_REGEX.captures(line).map_or(vec![], |caps| {
                    caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                });
                handle_unquoted_colon_in_description(line, captures, fixed_fm)?;
            }
            // Add other front matter regexes here
        } else if in_poem_body {
            poem_body_content.push_str(line);
            poem_body_content.push('\n');
        }
    }

    // Set the poem body after parsing all lines
    if !poem_body_content.is_empty() {
        fixed_fm.set_poem_body(poem_body_content);
    }

    Ok(())
}
