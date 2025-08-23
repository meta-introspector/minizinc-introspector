use asciicast::{Entry, EventType, Header};
use std::fs::File;
use std::io::{self, BufReader};
use serde_json::Deserializer;
use grex::RegExpBuilder;
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let file_path = "docs/asciicast1.cast";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Read the header
    let header_value = de.next().ok_or_else(|| anyhow!("Missing header"))?;
    let header: Header = serde_json::from_value(header_value.map_err(|e| anyhow!(e))?)?;

    println!("Asciicast Header:");
    println!("  Version: {}", header.version);
    println!("  Width: {}", header.width);
    println!("  Height: {}", header.height);
    if let Some(timestamp) = header.timestamp {
        println!("  Timestamp: {}", timestamp);
    }
    if let Some(duration) = header.duration {
        println!("  Duration: {}", duration);
    }
    if let Some(title) = header.title {
        println!("  Title: {}", title);
    }

    let mut event_count = 0;
    println!("\nProcessing events and generating regexes...");

    for value in de {
        let entry: Entry = serde_json::from_value(value.map_err(|e| anyhow!(e))?)?;
        event_count += 1;

        match entry.event_type {
            EventType::Output => {
                let regex = RegExpBuilder::from(&[&entry.event_data]).build();
                println!("Event {}: Output Regex: {}", event_count, regex);
            },
            EventType::Input => {
                // Ignore input events for now
            },
        }
    }
    println!("Total number of events processed: {}", event_count);

    Ok(())
}
