use std::{fs, path::PathBuf};
use anyhow::{Result, anyhow};
use clap::Parser;
use grex::RegExpBuilder;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input file
    #[arg(short, long)]
    input_file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct LineRegex {
    line: String,
    regex: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = fs::read_to_string(&args.input_file)
        .map_err(|e| anyhow!("Failed to read input file {:?}: {}", args.input_file, e))?;

    let mut unique_lines = content.lines().map(|s| s.to_string()).collect::<std::collections::HashSet<String>>();
    let mut line_regexes: Vec<LineRegex> = Vec::new();

    // GrexOptions is not directly used with RegExpBuilder::from().build()
    // let options = GrexOptions::default(); 

    for line in unique_lines.drain() {
        let regex = RegExpBuilder::from(&[&line]).build(); // Removed .map_err and ?
        line_regexes.push(LineRegex { line, regex });
    }

    let json_output = serde_json::to_string_pretty(&line_regexes)
        .map_err(|e| anyhow!("Failed to serialize regexes to JSON: {}", e))?;

    println!("{}", json_output);

    Ok(())
}