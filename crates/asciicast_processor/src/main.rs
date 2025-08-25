use asciicast::{Entry, EventType, Header};
use std::fs::File;
use std::io::{BufReader, Write, BufRead};
use serde_json::Deserializer;
use anyhow::{Result, anyhow};
use strip_ansi_escapes::strip;
use regex::Regex;


use gemini_utils::gemini_eprintln;

mod cli;
mod pattern_generator;
mod rust_parser;
mod raw_parser; // New module

use clap::Parser;

use cli::{Args, Commands};
use pattern_generator::{build_hierarchy, generate_poem_functions, map_to_ascii_names};
use rust_parser::extract_patterns_from_rust_file;
use raw_parser::check_raw_matches; // New use statement

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.input_file)?;
    let reader = BufReader::new(file);

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Read the header
    let header_value = de.next().ok_or_else(|| anyhow!("Missing header"))?;
    let header: Header = serde_json::from_value(header_value.map_err(|e: serde_json::Error| anyhow!(e))?)?;

    gemini_eprintln!("Asciicast Header:");
    gemini_eprintln!("  Version: :version:", version = header.version);
    gemini_eprintln!("  Width: :width:", width = header.width);
    gemini_eprintln!("  Height: :height:", height = header.height);
    if let Some(timestamp) = header.timestamp {
        gemini_eprintln!("  Timestamp: :timestamp:", timestamp = timestamp);
    }
    if let Some(duration) = header.duration {
        gemini_eprintln!("  Duration: :duration:", duration = duration);
    }
    if let Some(title) = header.title {
        gemini_eprintln!("  Title: :title:", title = title);
    }

    let mut all_events: Vec<Entry> = Vec::new();
    let mut event_count = 0;

    // Collect all events
    for value in de {
        let entry: Entry = serde_json::from_value(value.map_err(|e: serde_json::Error| anyhow!(e))?)?;
        all_events.push(entry);
        event_count += 1;
    }

    match args.command {
        Commands::Generate { limit, tail, steps, rust_output_file, ascii_names } => {
            let start_index = if let Some(tail_count) = tail {
                if tail_count >= all_events.len() {
                    0 // Process all if tail_count is greater than or equal to total events
                } else {
                    all_events.len() - tail_count
                }
            } else {
                0 // Start from beginning for --limit
            };

            #[allow(unused_variables)]
            let end_index = if let Some(tail_count) = tail {
                event_count // Process up to the end of collected events
            } else {
                limit.min(event_count) // Process up to limit or collected events
            };

            gemini_eprintln!("::sparkles::Processing events and collecting cleaned output (limited to :limit:)...::sparkles::", limit = limit);

            let mut cleaned_output_lines: Vec<String> = Vec::new();
            for i in start_index..end_index {
                let entry = &all_events[i];
                match entry.event_type {
                    EventType::Output => {
                        let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();
			gemini_eprintln!("DEBUG: Cleaned output data: :cleaned_data:", cleaned_data = cleaned_data);
                        cleaned_output_lines.push(cleaned_data);
                    },
                    EventType::Input => {
                        // Ignore input events for now
                    },
                }
            }

            gemini_eprintln!("Total number of events processed: :event_count:", event_count = event_count);

            let hierarchy = build_hierarchy(cleaned_output_lines, &steps);

            let generated_code = generate_poem_functions(&hierarchy, "root", 0);

            let mut output_file = File::create(&rust_output_file)?;
            output_file.write_all(generated_code.to_string().as_bytes())?;

            gemini_eprintln!("Generated Rust code written to: :file_path:", file_path = format!("{:?}", rust_output_file));
            return Ok(())
        },
        Commands::Analyze { generated_rust_file } => {
            gemini_eprintln!("Entering Analyze mode with generated Rust file: :file_path:", file_path = format!("{:?}", generated_rust_file));

            let patterns = extract_patterns_from_rust_file(&generated_rust_file)?;
            gemini_eprintln!("Extracted :count: patterns from :file_path:", count = patterns.len(), file_path = format!("{:?}", generated_rust_file));
            for pattern in patterns {
                gemini_eprintln!("  - :pattern:", pattern = pattern);
            }
            // TODO: Implement analysis logic here
            // 2. Match against all_events output lines
            // 3. Calculate match percentage
            // 4. Identify unmatched lines
            // 5. Perform bag-of-words and cosine similarity on unmatched lines
            // 6. Generate new regexes for unmatched lines
            return Ok(())
        },
        Commands::Filter { limit, regex, context, occurrences } => {
            let escaped_regex = regex::escape(&regex);
            let filter_regex = Regex::new(&escaped_regex)?;

            // Check if regex is present in raw input
            let raw_match_found = check_raw_matches(&args.input_file, &regex)?;

            let mut matched_lines_with_context: Vec<String> = Vec::new();
            let mut last_match_index: Option<usize> = None;
            let mut matches_found = 0; // Track number of matches

            let mut cleaned_output_lines: Vec<String> = Vec::new();
            let mut processed_events_count = 0; // Track processed events

            for entry in &all_events {
                if processed_events_count >= limit {
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
		        gemini_eprintln!("DEBUG: Line contains regex string: :contains_result:", contains_result = line.contains(&regex));
		        gemini_eprintln!("DEBUG: Matching regex :regex_val: against line :line_display:. Result: :match_result:",
		            regex_val = regex,
		            line_display = line,
		            match_result = filter_regex.is_match(line)
		        );
		gemini_eprintln!("DEBUG: Line: :i: :line:", i = i, line=line);
                if filter_regex.is_match(line) {
                    processed_match_found = true;
                    if let Some(max_occurrences) = occurrences {
                        if matches_found >= max_occurrences {
                            break; // Stop if occurrences limit is reached
                        }
                    }

		    gemini_eprintln!("DEBUG: Regex matched line: :i: :line:", i = i, line=line);
                    matches_found += 1;

                    // Add context before the match
                    let start_index = if i >= context { i - context } else { 0 };
                    for j in start_index..i {
                        if last_match_index.is_none() || j >= last_match_index.unwrap() + context + 1 {
                            matched_lines_with_context.push(format!("{} {}", j, cleaned_output_lines[j]));
                        }
                    }
                    // Add the matching line
                    matched_lines_with_context.push(format!("{} {}", i, line));
                    last_match_index = Some(i);
                } else if let Some(last_idx) = last_match_index {
                    // Add context after the match
                    if i <= last_idx + context {
                        matched_lines_with_context.push(format!("{} {}", i, line));
                    }
                }
            }

            // Panic logic
            if raw_match_found && !processed_match_found {
                // Find the first raw event that matches the regex
                let mut problematic_raw_entry: Option<&Entry> = None;
                let mut problematic_cleaned_data: Option<String> = None;

                for entry in &all_events {
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

                // panic!("Discrepancy detected between raw and processed output.");
            }

            for line in matched_lines_with_context {
                println!("{}", line);
            }
            return Ok(())
        },
        Commands::CountRaw { regex } => {
            let file = File::open(&args.input_file)?;
            let reader = BufReader::new(file);
            let filter_regex = Regex::new(&regex)?;

            let mut matches_found = 0;
	  
            gemini_eprintln!("Searching raw input for pattern: ':regex::newline:", regex = regex);

            for (i, line_result) in reader.lines().enumerate() {
                let line = line_result?.to_string(); // Convert to String
                if filter_regex.is_match(&line) {
                    matches_found += 1;
                    gemini_eprintln!("RAW MATCH (Line :line_num:): :line:", line_num = i + 1, line = line);
                }
            }
            gemini_eprintln!("::newline:: Total raw matches found: :matches_found:", matches_found = matches_found);

            return Ok(())
        },
    }

    }
