use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Size of each chunk for word embeddings
    #[arg(short, long, default_value_t = 100)]
    pub chunk_size: usize,
}