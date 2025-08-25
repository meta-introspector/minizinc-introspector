use anyhow::Result;
use std::fs::File;
use std::io::Write;
use asciicast::{Entry, EventType};
use strip_ansi_escapes::strip;

use crate::cli::GenerateArgs;
use crate::pattern_generator::{build_hierarchy, generate_poem_functions};
use gemini_utils::gemini_eprintln;

pub fn handle_generate_command(
    args: &GenerateArgs,
    all_events: &[Entry],
    event_count: usize,
) -> Result<()> {
    let GenerateArgs { limit, tail, steps, rust_output_file, ascii_names: _ } = args; // Fixed: ascii_names: _

    let start_index = if let Some(tail_count) = tail {
        if *tail_count >= all_events.len() {
            0 // Process all if tail_count is greater than or equal to total events
        } else {
            all_events.len() - *tail_count
        }
    } else {
        0 // Start from beginning for --limit
    };

    #[allow(unused_variables)]
    let end_index = if let Some(tail_count) = tail {
        event_count // Process up to the end of collected events
    } else {
        (*limit).min(event_count) // Process up to limit or collected events
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
    Ok(())
}
