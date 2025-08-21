use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generates word embedding data for MiniZinc
    GenerateData {
        /// Size of each chunk for word embeddings
        #[arg(short, long, default_value_t = 100)]
        chunk_size: usize,

        /// Optional: Path to a specific file or directory to process. If not provided, the current directory will be processed.
        #[arg(long)]
        input_path: Option<std::path::PathBuf>,
    },
    /// Runs the hf-validator binary to analyze a Rust project
    RunHfValidator {
        /// Path to the Rust project to analyze
        #[arg(long)]
        project_path: std::path::PathBuf,
        /// Output directory for the generated Hugging Face dataset
        #[arg(long)]
        output_path: std::path::PathBuf,
    },
    /// Inspects the schema of a Parquet file
    InspectParquet {
        /// Path to the Parquet file to inspect
        #[arg(long)]
        file_path: std::path::PathBuf,
    },
    /// Looks up the embedding for a given word in the word_embeddings.parquet file
    LookupEmbedding {
        /// The word to look up
        #[arg(long)]
        word: String,
    },
    /// Inspects the schema of a Parquet file
    InspectParquetSchema {
        /// Path to the Parquet file to inspect
        #[arg(long)]
        file_path: std::path::PathBuf,
    },
}
