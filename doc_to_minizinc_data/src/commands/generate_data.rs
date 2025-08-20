use crate::prelude::*; // Use the prelude

pub fn handle_generate_data_command(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Generate logical relationships from simulated WordNet once
    let simulated_wordnet_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/simulated_wordnet.txt");
    let all_relations = generate_wordnet_constraints(&simulated_wordnet_path)?;

    data_generation::generate_data(args, all_relations)?;

    Ok(())
}
