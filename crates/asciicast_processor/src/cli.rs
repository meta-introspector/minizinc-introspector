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
    Generate {
        /// Limit the number of events to process from the beginning
        #[arg(long, default_value_t = 10, conflicts_with = "tail")]
        limit: usize,
        /// Process only the last N events
        #[arg(long, conflicts_with = "limit")]
        tail: Option<usize>,
        /// Steps for hierarchical grouping (e.g., --steps 5,3,1)
        #[arg(long, value_delimiter = ',', default_values_t = [5, 3, 1])]
        steps: Vec<usize>,
        /// Output file to save the generated Rust code
        #[arg(long)]
        rust_output_file: PathBuf,
        /// Enable ASCII naming for Unicode characters and ANSI sequences
        #[arg(long)]
        ascii_names: bool,
    },
    /// Analyzes an asciicast file using previously generated Rust code
    Analyze {
        /// Path to the previously generated Rust code file (.rs)
        #[arg(long)]
        generated_rust_file: PathBuf,
    },
    /// Filters asciicast output by regex and shows context
    Filter {
        /// Limit the number of events to process from the beginning
        #[arg(long, default_value_t = 10)]
        limit: usize,
        /// Regex pattern to filter lines
        #[arg(long)]
        regex: String,
        /// Show N lines of context around matching lines
        #[arg(short = 'C', long, default_value_t = 0)]
        context: usize,
        /// Limit the number of matching occurrences to find
        #[arg(long)]
        occurrences: Option<usize>,
    },
    /// Counts raw matches in the input file
    CountRaw {
        /// Regex pattern to count raw matches
        #[arg(long)]
        regex: String,
    },
