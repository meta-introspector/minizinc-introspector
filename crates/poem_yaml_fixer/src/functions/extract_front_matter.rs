// This file contains the extract_front_matter function.
// It is responsible for parsing the YAML front matter delimiters and extracting
// the front matter content and the raw poem body from the input text.

use anyhow::Result;

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

    if fm_start == -1 {
        // No front matter delimiters found, treat entire content as poem body
        return Ok((-1, -1, String::new(), content.to_string()));
    }

    if fm_start != 0 {
        // If the first --- is not on the first line, treat the whole file as body
        return Ok((-1, -1, String::new(), content.to_string()));
    }

    let front_matter_str_for_parsing: String;
    let extracted_poem_body_from_fm: String;

    if fm_end == -1 {
        // Case: Missing closing ---. Treat everything after fm_start as front matter.
        front_matter_str_for_parsing = lines[(fm_start + 1) as usize..].join("\n");
        extracted_poem_body_from_fm = String::new(); // No separate poem body if closing --- is missing
    } else {
        // Case: Both --- delimiters found.
        let front_matter_lines_slice = &lines[(fm_start + 1) as usize .. fm_end as usize];
        let mut temp_front_matter_str = String::new();
        let mut temp_extracted_poem_body = String::new();
        let mut in_poem_body_in_fm = false;

        // Manually process front matter lines to extract poem_body if present
        for line in front_matter_lines_slice.iter() {
            if line.trim().starts_with("poem_body: |") {
                in_poem_body_in_fm = true;
            } else if in_poem_body_in_fm {
                if line.starts_with(" ") {
                    temp_extracted_poem_body.push_str(line.trim_start());
                    temp_extracted_poem_body.push('\n');
                } else {
                    in_poem_body_in_fm = false;
                    temp_front_matter_str.push_str(line);
                    temp_front_matter_str.push('\n');
                }
            } else {
                temp_front_matter_str.push_str(line);
                temp_front_matter_str.push('\n');
            }
        }
        front_matter_str_for_parsing = temp_front_matter_str;
        extracted_poem_body_from_fm = temp_extracted_poem_body;
    }

    Ok((fm_start, fm_end, front_matter_str_for_parsing, extracted_poem_body_from_fm))
}