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
    /// Compile all terms from the hierarchical term index into a single regex
    CompileTermsRegex,
    /// Recognize terms in a given text using the compiled regex
    RecognizeText(commands::recognize_text::RecognizeTextArgs),
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::BatchQuery(args) => commands::batch_query::run_batch_query(args.clone()),
        Commands::CompileTermsRegex => {
            let (regex_pattern, filtered_terms) = commands::compile_terms_regex::run_compile_terms_regex()?;
            commands::compile_terms_regex::generate_recognizer_file(regex_pattern, filtered_terms)?;
            Ok(())
        },
        Commands::RecognizeText(args) => commands::recognizer::run_recognize_text(args.clone()),
    };

    result
}
