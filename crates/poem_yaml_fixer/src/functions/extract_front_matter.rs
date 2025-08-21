// This file contains the extract_front_matter function.
// It is responsible for parsing the YAML front matter delimiters and extracting
// the front matter content and the raw poem body from the input text.

//use std::path::PathBuf;
use anyhow::{Result, anyhow};

pub fn extract_front_matter(lines: &mut Vec<&str>, content: &str) -> Result<(isize, isize, String, String)> {
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
