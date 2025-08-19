use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Size of each chunk for word embeddings
    #[arg(short, long, default_value_t = 100)]
    pub chunk_size: usize,

    /// Optional: Path to a specific file or directory to process. If not provided, the current directory will be processed.
    #[arg(long)]
    pub input_path: Option<std::path::PathBuf>,
}