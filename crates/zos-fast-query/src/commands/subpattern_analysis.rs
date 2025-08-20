use clap::Parser;
use crate::recognizer::TermRecognizer;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct SubpatternAnalysisArgs {
    /// The text to analyze for subpattern relationships
    #[arg(short, long)]
    pub text: String,
}

pub fn run_subpattern_analysis(args: SubpatternAnalysisArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("ZOS Fast Query Tool - Subpattern Analysis");

    let recognizer = TermRecognizer::new()?;

    println!("\nText to analyze: \"{}\"", args.text);
    let recognized_terms = recognizer.recognize_terms(&args.text);

    println!("\nSubpattern Relationships:");
    if recognized_terms.is_empty() {
        println!("  No terms recognized for analysis.");
    } else {
        let mut found_relationships = false;
        for i in 0..recognized_terms.len() {
            for j in 0..recognized_terms.len() {
                if i == j { continue; }

                let term1 = &recognized_terms[i];
                let term2 = &recognized_terms[j];

                if term1.contains(term2) {
                    println!("  - \"{}\" contains \"{}\"", term1, term2);
                    found_relationships = true;
                }
            }
        }
        if !found_relationships {
            println!("  No subpattern relationships found.");
        }
    }

    Ok(())
}

