use regex::Regex;
use std::collections::HashSet;

// Include the generated recognizer file
#[allow(dead_code)] // Allow dead code for now, as it's generated
#[allow(unused_imports)] // Allow unused imports for now
#[allow(clippy::all)] // Allow all clippy lints for generated code
mod generated_recognizer;

pub struct TermRecognizer {
    // No longer stores regex and terms directly, they are static
}

impl TermRecognizer {
    /// Creates a new TermRecognizer. It now uses the precompiled static assets.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Access the static regex and terms from the generated module
        let _ = &*generated_recognizer::RECOGNIZER_REGEX; // Ensure it's initialized
        let _ = &*generated_recognizer::RECOGNIZER_TERMS; // Ensure it's initialized
        Ok(Self {})
    }

    /// Recognizes terms in the given text and returns a vector of matched terms.
    pub fn recognize_terms(&self, text: &str) -> Vec<String> {
        generated_recognizer::RECOGNIZER_REGEX.find_iter(text)
            .map(|m| m.as_str().to_string())
            .filter(|s| generated_recognizer::RECOGNIZER_TERMS.contains(s)) // Ensure the matched string is one of our original terms
            .collect()
    }
}