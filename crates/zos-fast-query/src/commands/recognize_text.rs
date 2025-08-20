use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use clap::Parser;

use crate::recognizer::TermRecognizer;
use crate::commands::compile_terms_regex::run_compile_terms_regex;



#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct RecognizeTextArgs {
    /// The text to recognize terms in
    #[arg(short, long)]
    pub text: String,

    /// Path to the hierarchical term index JSON file (needed to get the terms for the recognizer)
    #[arg(short = 'i', long, default_value_os = "/data/data/com.termux/files/home/storage/github/hierarchical_term_index.json")]
    pub hierarchical_index: PathBuf,
}

pub fn run_recognize_text(args: RecognizeTextArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("ZOS Fast Query Tool - Recognizing Terms in Text");

    // 1. Compile terms to regex and get the list of terms
    let regex_pattern = run_compile_terms_regex()?;

    // 2. Load the hierarchical term index to get the original terms (needed for TermRecognizer::new)
    println!("Loading hierarchical term index from: {:?}", args.hierarchical_index);
    let cached_data = fs::read_to_string(&args.hierarchical_index)?;
    let hierarchical_term_index: HashMap<String, HashMap<PathBuf, usize>> = serde_json::from_str(&cached_data)?;
    let all_terms: Vec<String> = hierarchical_term_index.keys().cloned().collect();

    // 3. Create the TermRecognizer
    let recognizer = TermRecognizer::new(regex_pattern, all_terms)?;

    // 4. Recognize terms in the provided text
    println!("\nText to analyze: \"{}\"", args.text);
    let recognized_terms = recognizer.recognize_terms(&args.text);

    // 5. Print recognized terms
    println!("\nRecognized Terms:");
    if recognized_terms.is_empty() {
        println!("  No terms recognized.");
    } else {
        for term in recognized_terms {
            println!("  - {}", term);
        }
    }

    Ok(())
}
