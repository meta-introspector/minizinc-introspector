use crate::functions::types::FixedFrontMatter;
use crate::functions::parse_front_matter_fields::parse_front_matter_fields;
// This module contains the logic for parsing .archeology.md files.

//use crate::functions::types::RawFrontMatter;
use anyhow::Result;
use std::fs;
use std::path::Path;
#[cfg(test)]
use crate::functions::types::FixedFrontMatter;
use crate::functions::extract_front_matter::extract_front_matter;

use poem_traits::{RegexConfig, FunctionRegistry};
//use crate::functions::types::RawFrontMatter;
pub fn parse_archeology_file(
    path: &Path,
    regex_config: &RegexConfig,
    function_registry: &FunctionRegistry,
) -> Result<Vec<FixedFrontMatter>> {
    let content = fs::read_to_string(path)?;
    let revisions: Vec<&str> = content.split("\n\n---\n\n").collect();

    let mut recovered_front_matters = Vec::new();

    for revision_str in revisions {
        if revision_str.trim().is_empty() {
            continue;
        }

        // Remove the header
        let mut lines: Vec<&str> = revision_str.lines().collect();
        if !lines.is_empty() && lines[0].starts_with("## Lost Revision from commit") {
            lines.remove(0);
            while !lines.is_empty() && lines[0].trim().is_empty() {
                lines.remove(0);
            }
        }

        let revision_content = lines.join("\n");
        let mut revision_lines: Vec<&str> = revision_content.lines().collect();

        let (_fm_start, fm_end, front_matter_str, poem_body_from_fm) = extract_front_matter(&mut revision_lines, &revision_content)?;

        let mut fixed_fm = FixedFrontMatter::default();
        if !front_matter_str.is_empty() {
            parse_front_matter_fields(
                &front_matter_str,
                &mut fixed_fm,
                regex_config,
                function_registry,
            )?;
        }

        let poem_body = if !poem_body_from_fm.is_empty() {
            poem_body_from_fm
        } else if fm_end != -1 {
            revision_lines[(fm_end + 1) as usize..].join("\n")
        } else {
            revision_content.clone()
        };

        if front_matter_str.is_empty() {
            fixed_fm.poem_body = Some(revision_content);
        } else {
            fixed_fm.poem_body = Some(poem_body);
        }

        recovered_front_matters.push(fixed_fm);
    }

    Ok(recovered_front_matters)
}