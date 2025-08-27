use anyhow::Result;
use std::fs::File;
use std::io::Write;
use asciicast::{Entry, EventType};
use strip_ansi_escapes::strip;

use crate::cli::ExtractLinesArgs;
use gemini_utils::gemini_eprintln;

pub fn handle_extract_lines_command(
    args: &ExtractLinesArgs,
    all_events: &[Entry],
) -> Result<()> {
    let ExtractLinesArgs { output_file } = args;

    let mut file = File::create(output_file)?;

    gemini_eprintln!("Extracting lines to: :file_path:", file_path = format!("{:?}", output_file));

    for entry in all_events {
        if let EventType::Output = entry.event_type {
            let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();
            file.write_all(cleaned_data.as_bytes())?;
            file.write_all(b"\n")?; // Add a newline after each extracted line
        }
    }

    gemini_eprintln!("Extraction complete.");

    Ok(())
}
