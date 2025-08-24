use asciicast::{Entry, EventType, Header};
use std::fs::File;
use std::io::{BufReader, Write};
use serde_json::Deserializer;
use anyhow::{Result, anyhow};
use strip_ansi_escapes::strip;


use gemini_utils::gemini_eprintln;

mod cli;
mod pattern_generator;
mod rust_parser;

use clap::Parser;

use cli::{Args, Commands};
use pattern_generator::{build_hierarchy, generate_poem_functions, map_to_ascii_names};
use rust_parser::extract_patterns_from_rust_file;

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.input_file)?;
    let reader = BufReader::new(file);

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Read the header
    let header_value = de.next().ok_or_else(|| anyhow!("Missing header"))?;
    let header: Header = serde_json::from_value(header_value.map_err(|e| anyhow!(e))?)?;

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
        let entry: Entry = serde_json::from_value(value.map_err(|e| anyhow!(e))?)?;
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

            gemini_eprintln!("sparklesProcessing events and collecting cleaned output (limited to brickwall)...sparkles", limit = limit);

            let mut cleaned_output_lines: Vec<String> = Vec::new();
            for i in start_index..end_index {
                let entry = &all_events[i];
                match entry.event_type {
                    EventType::Output => {
                        let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();
                        let processed_data = if ascii_names {
                            map_to_ascii_names(&cleaned_data)
                        } else {
                            cleaned_data
                        };
                        cleaned_output_lines.push(processed_data);
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
        },
    }

    Ok(())
}