use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the asciicast input file
    pub input_file: PathBuf, // Added input file argument

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generates Rust code from an asciicast file
    Generate(GenerateArgs),
    /// Analyzes an asciicast file using previously generated Rust code
    Analyze(AnalyzeArgs),
    /// Filters asciicast output by regex and shows context
    Filter(FilterArgs),
    /// Counts raw matches in the input file
    #[command(name = "count-raw")]
    CountRaw(CountRawArgs),
}

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Limit the number of events to process from the beginning
    #[arg(long, default_value_t = 10, conflicts_with = "tail")]
    pub limit: usize,
    /// Process only the last N events
    #[arg(long, conflicts_with = "limit")]
    pub tail: Option<usize>,
    /// Steps for hierarchical grouping (e.g., --steps 5,3,1)
    #[arg(long, value_delimiter = ',', default_values_t = [5, 3, 1])]
    pub steps: Vec<usize>,
    /// Output file to save the generated Rust code
    #[arg(long)]
    pub rust_output_file: PathBuf,
    /// Enable ASCII naming for Unicode characters and ANSI sequences
    #[arg(long)]
    pub ascii_names: bool,
}

#[derive(Parser, Debug)]
pub struct AnalyzeArgs {
    /// Path to the previously generated Rust code file (.rs)
    #[arg(long)]
    pub generated_rust_file: PathBuf,
}

#[derive(Parser, Debug)]
pub struct FilterArgs {
    /// Limit the number of events to process from the beginning
    #[arg(long, default_value_t = 10)]
    pub limit: usize,
    /// Regex pattern to filter lines
    #[arg(long)]
    pub regex: String,
    /// Show N lines of context around matching lines
    #[arg(short = 'C', long, default_value_t = 0)]
    pub context: usize,
    /// Limit the number of matching occurrences to find
    #[arg(long)]
    pub occurrences: Option<usize>,
    // Add input_file here as it's needed by the handler
    #[arg(skip)]
    pub input_file: PathBuf,
}

#[derive(Parser, Debug)]
pub struct CountRawArgs {
    /// Regex pattern to count raw matches
    #[arg(long)]
    pub regex: String,
    // Add input_file here as it's needed by the handler
    #[arg(skip)]
    pub input_file: PathBuf,
}
