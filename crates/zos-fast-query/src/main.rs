use clap::{Parser, Subcommand};

mod commands;
mod recognizer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Perform a batch query on the hierarchical term index
    BatchQuery(commands::batch_query::BatchQueryArgs),
    /// Recognize terms in a given text using the compiled regex
    RecognizeText(commands::recognize_text::RecognizeTextArgs),
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::BatchQuery(args) => commands::batch_query::run_batch_query(args.clone()),
        Commands::RecognizeText(args) => commands::recognize_text::run_recognize_text(args.clone()),
    };

    result
}