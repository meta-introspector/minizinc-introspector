use anyhow::Result;
use regex::Regex;
use asciicast::{Entry, EventType};
use strip_ansi_escapes::strip;

use crate::cli::FilterArgs;
use crate::raw_parser::check_raw_matches;
use gemini_utils::gemini_eprintln;

pub fn handle_filter_command(
    args: &FilterArgs,
    all_events: &[Entry],
) -> Result<()> {
    let FilterArgs { limit, regex, context, occurrences, input_file: _ } = args;

    let escaped_regex = regex::escape(&regex);
    let filter_regex = Regex::new(&escaped_regex)?;

    // Check if regex is present in raw input
    let raw_match_found = check_raw_matches(&args.input_file, &regex)?;

    let mut matched_lines_with_context: Vec<String> = Vec::new();
    let mut last_match_index: Option<usize> = None;
    let mut matches_found = 0; // Track number of matches

    let mut cleaned_output_lines: Vec<String> = Vec::new();
    let mut processed_events_count = 0; // Track processed events

    for entry in all_events {
        if processed_events_count >= *limit {
            break; // Stop if event limit is reached
        }
        eprintln!("DEBUG: Raw event data: {:?}", entry); // Log raw event data
        match entry.event_type {
            EventType::Output => {
                let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();
                gemini_eprintln!("DEBUG: Cleaned output data: :cleaned_data:", cleaned_data = cleaned_data);
                cleaned_output_lines.push(cleaned_data);
            },
            EventType::Input => {
                let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();

                gemini_eprintln!("DEBUG: Cleaned output data: :cleaned_data:", cleaned_data = cleaned_data);
                cleaned_output_lines.push(format!("INPUT: {}", cleaned_data)); // Prefix input events
            },
        }
        processed_events_count += 1;
    }

    let mut processed_match_found = false;
    for (i, line) in cleaned_output_lines.iter().enumerate() {
        gemini_eprintln!("DEBUG: Regex: :regex_val:", regex_val = regex);
        gemini_eprintln!("DEBUG: Escaped Regex: :escaped_regex_val:", escaped_regex_val = escaped_regex);
        gemini_eprintln!("DEBUG: Line (raw): :line_raw_debug:", line_raw_debug = format!("{:?}", line));
        gemini_eprintln!("DEBUG: Line (display): :line_display:", line_display = line);
        gemini_eprintln!("DEBUG: Line (bytes): :line_bytes_debug:", line_bytes_debug = format!("{:?}", line.as_bytes()));
        gemini_eprintln!("DEBUG: Line contains regex string: :contains_result:", contains_result = line.contains(regex.as_str()));
        gemini_eprintln!("DEBUG: Matching regex :regex_val: against line :line_display:. Result: :match_result:",
            regex_val = regex,
            line_display = line,
            match_result = filter_regex.is_match(line)
        );
        gemini_eprintln!("DEBUG: Line: :i: :line:", i = i, line=line);
        if filter_regex.is_match(line) {
            processed_match_found = true;
            if let Some(max_occurrences) = occurrences {
                if matches_found >= *max_occurrences {
                    break; // Stop if occurrences limit is reached
                }
            }

            gemini_eprintln!("DEBUG: Regex matched line: :i: :line:", i = i, line=line);
            matches_found += 1;

            // Add context before the match
            let start_index = if i >= *context { i - *context } else { 0 };
            for j in start_index..i {
                if last_match_index.is_none() || j >= last_match_index.unwrap() + *context + 1 {
                    matched_lines_with_context.push(format!("{} {}", j, cleaned_output_lines[j]));
                }
            }
            // Add the matching line
            matched_lines_with_context.push(format!("{} {}", i, line));
            last_match_index = Some(i);
        } else if let Some(last_idx) = last_match_index {
            // Add context after the match
            if i <= last_idx + *context {
                matched_lines_with_context.push(format!("{} {}", i, line));
            }
        }
    }

    // Panic logic
    if raw_match_found && !processed_match_found {
        // Find the first raw event that matches the regex
        let mut problematic_raw_entry: Option<&Entry> = None;
        let mut problematic_cleaned_data: Option<String> = None;

        for entry in all_events {
            let raw_data_str = String::from_utf8_lossy(entry.event_data.as_bytes());
            if filter_regex.is_match(&raw_data_str) {
                problematic_raw_entry = Some(entry);
                problematic_cleaned_data = Some(String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string());
                break; // Found the first one, no need to search further
            }
        }

        gemini_eprintln!("ERROR: String found in raw input but not in processed output!");
        gemini_eprintln!("  Regex: :regex:", regex = regex);
        gemini_eprintln!("  Raw match found: :raw_match_found:", raw_match_found = raw_match_found);
        gemini_eprintln!("  Processed match found: :processed_match_found:", processed_match_found = processed_match_found);

        if let Some(entry) = problematic_raw_entry {
            gemini_eprintln!("  Problematic Raw Event Data: :raw_event_data:", raw_event_data = format!("{:?}", entry.event_data));
            if let Some(cleaned_data) = problematic_cleaned_data {
                gemini_eprintln!("  Problematic Cleaned Data (UTF-8): :cleaned_data:", cleaned_data = cleaned_data);
                gemini_eprintln!("  Problematic Cleaned Data (Bytes): :cleaned_data_bytes:", cleaned_data_bytes = format!("{:?}", cleaned_data.as_bytes()));
            }
        }

        panic!("Discrepancy detected between raw and processed output.");
    }

    for line in matched_lines_with_context {
        println!("{}", line);
    }
    Ok(())
}