use asciicast::{Entry, Header};
use std::fs::File;
use std::io::{BufReader};
use serde_json::Deserializer;
use anyhow::{Result, anyhow};

pub struct AsciicastData {
    pub header: Header,
    pub events: Vec<Entry>,
    pub event_count: usize,
}

pub fn read_asciicast_file(input_file: &str) -> Result<AsciicastData> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Read the header
    let header_value = de.next().ok_or_else(|| anyhow!("Missing header"))?;
    let header: Header = serde_json::from_value(header_value.map_err(|e: serde_json::Error| anyhow!(e))?)?;

    let mut all_events: Vec<Entry> = Vec::new();
    let mut event_count = 0;

    // Collect all events
    for value in de {
        let entry: Entry = serde_json::from_value(value.map_err(|e: serde_json::Error| anyhow!(e))?)?;
        all_events.push(entry);
        event_count += 1;
    }

    Ok(AsciicastData {
        header,
        events: all_events,
        event_count,
    })
}