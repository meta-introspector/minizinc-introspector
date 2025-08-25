use anyhow::Result;
use clap::Parser;

use gemini_utils::gemini_eprintln;

mod cli;
mod pattern_generator;
mod rust_parser;
mod raw_parser;
mod asciicast_reader; // New module
mod commands; // New module

use cli::{Args, Commands};
use asciicast_reader::read_asciicast_file; // New use statement
use commands::generate::handle_generate_command;
use commands::analyze::handle_analyze_command;
use commands::filter::handle_filter_command;
use commands::count_raw::handle_count_raw_command;

fn main() -> Result<()> {
    let args = Args::parse();

    // Read asciicast data
    let asciicast_data = read_asciicast_file(&args.input_file.to_string_lossy())?;

    gemini_eprintln!("Asciicast Header:");
    gemini_eprintln!("  Version: :version:", version = asciicast_data.header.version);
    gemini_eprintln!("  Width: :width:", width = asciicast_data.header.width);
    gemini_eprintln!("  Height: :height:", height = asciicast_data.header.height);
    if let Some(timestamp) = asciicast_data.header.timestamp {
        gemini_eprintln!("  Timestamp: :timestamp:", timestamp = timestamp);
    }
    if let Some(duration) = asciicast_data.header.duration {
        gemini_eprintln!("  Duration: :duration:", duration = duration);
    }
    if let Some(title) = asciicast_data.header.title {
        gemini_eprintln!("  Title: :title:", title = title);
    }

    match args.command {
        Commands::Generate(generate_args) => {
            handle_generate_command(&generate_args, &asciicast_data.events, asciicast_data.event_count)?;
        },
        Commands::Analyze(analyze_args) => {
            handle_analyze_command(&analyze_args)?;
        },
        Commands::Filter(mut filter_args) => {
            // Pass input_file to filter_args as it's needed by check_raw_matches
            filter_args.input_file = args.input_file;
            handle_filter_command(&filter_args, &asciicast_data.events)?;
        },
        Commands::CountRaw(mut count_raw_args) => {
            // Pass input_file to count_raw_args as it's needed by the handler
            count_raw_args.input_file = args.input_file;
            handle_count_raw_command(&count_raw_args)?;
        },
    }

    Ok(())
}
