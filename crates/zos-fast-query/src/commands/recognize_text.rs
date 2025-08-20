//use std::fs;
//use std::path::PathBuf;


use clap::Parser;

use crate::recognizer::TermRecognizer;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct RecognizeTextArgs {
    /// The text to recognize terms in
    #[arg(short, long)]
    pub text: String,
}

pub fn run_recognize_text(args: RecognizeTextArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("ZOS Fast Query Tool - Recognizing Terms in Text");

    // Create the TermRecognizer (it now loads terms internally)
    let recognizer = TermRecognizer::new()?;

    // Recognize terms in the provided text
    println!("\nText to analyze: \"{}\"", args.text);
    let recognized_terms = recognizer.recognize_terms(&args.text);

    // Print recognized terms
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
