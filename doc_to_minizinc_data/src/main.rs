use clap::Parser;
use doc_to_minizinc_data::cli::Args;
use doc_to_minizinc_data::wordnet_processing::generate_wordnet_constraints;
use doc_to_minizinc_data::data_generation::generate_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Generate logical relationships from simulated WordNet once
    let simulated_wordnet_path = std::path::PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/simulated_wordnet.txt");
    let all_relations = generate_wordnet_constraints(&simulated_wordnet_path)?;

    generate_data(args, all_relations)?;

    Ok(())
}