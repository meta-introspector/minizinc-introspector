use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::fs;

use crate::functions::extract_front_matter::extract_front_matter;
use crate::functions::parse_front_matter_fields::parse_front_matter_fields;
use crate::functions::types::FixedFrontMatter;

pub fn parse_poem_file_direct(file_path: &PathBuf) -> Result<(FixedFrontMatter, String)> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| anyhow!("Failed to read file {}: {}", file_path.display(), e))?;

    let mut lines: Vec<&str> = content.lines().collect();
    let (_fm_start, _fm_end, front_matter_str_for_parsing, extracted_poem_body_from_fm) =
        extract_front_matter(&mut lines, &content)?;

    let mut fixed_fm = FixedFrontMatter::default(); // FixedFrontMatter needs to derive Default
    parse_front_matter_fields(&front_matter_str_for_parsing, &mut fixed_fm)?;

    Ok((fixed_fm, extracted_poem_body_from_fm))
}
