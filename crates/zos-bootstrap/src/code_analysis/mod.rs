pub mod ast_to_numerical_vector_converter;
pub mod constant_analyzer;
pub mod constant_usage_proof;
pub mod dzn_data_generator;
pub mod minizinc_model_generator;
pub mod minizinc_param_generator;
pub mod numerical_vector_generator;
pub mod numerical_vector_to_llm_instructions;
pub mod string_extractor;
//pub mod minizinc_model_generator_helpers; // This is a directory

use std::path::{PathBuf, Path};
use std::fs;
use std::collections::{HashSet, HashMap};
//use syn::{parse_file, Item};
//use quote::quote; // Often used with syn
use regex::Regex; // For keyword extraction

pub struct CodeMatch {
    pub file_path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub code_snippet: String,
    pub similarity_score: f64,
}

#[derive(Debug)]
pub enum CodeAnalysisError {
    ParseError(String),
    IoError(std::io::Error),
    WalkDirError(walkdir::Error),
    // Add more error types as needed
}

impl From<std::io::Error> for CodeAnalysisError {
    fn from(err: std::io::Error) -> Self {
        CodeAnalysisError::IoError(err)
    }
}

impl From<walkdir::Error> for CodeAnalysisError {
    fn from(err: walkdir::Error) -> Self {
        CodeAnalysisError::WalkDirError(err)
    }
}

// Helper function to tokenize and normalize text
fn tokenize(text: &str) -> HashSet<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

// Expanded synonym map
fn get_synonyms() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    // Core Concepts
    map.insert("code".to_string(), vec!["source".to_string(), "program".to_string(), "script".to_string(), "text".to_string(), "file_content".to_string(), "snippet".to_string()]);
    map.insert("analysis".to_string(), vec!["parse".to_string(), "inspect".to_string(), "examine".to_string(), "process".to_string(), "evaluate".to_string(), "understand".to_string(), "interpret".to_string(), "scan".to_string(), "traverse".to_string(), "walk".to_string(), "explore".to_string()]);
    map.insert("duplicate".to_string(), vec!["similar".to_string(), "copy".to_string(), "redundant".to_string(), "identical".to_string(), "matching".to_string(), "clone".to_string(), "replicate".to_string()]);
    map.insert("find".to_string(), vec!["locate".to_string(), "search".to_string(), "discover".to_string(), "identify".to_string(), "detect".to_string(), "uncover".to_string()]);
    map.insert("function".to_string(), vec!["fn".to_string(), "method".to_string(), "procedure".to_string(), "routine".to_string(), "subroutine".to_string(), "callable".to_string()]);
    map.insert("structure".to_string(), vec!["struct".to_string(), "class".to_string(), "object".to_string(), "data_structure".to_string(), "record".to_string()]);
    map.insert("implement".to_string(), vec!["impl".to_string(), "trait".to_string(), "interface".to_string(), "define".to_string(), "provide".to_string()]);
    map.insert("error".to_string(), vec!["err".to_string(), "result".to_string(), "failure".to_string(), "issue".to_string(), "problem".to_string(), "exception".to_string()]);
    map.insert("path".to_string(), vec!["filepath".to_string(), "directory".to_string(), "dir".to_string(), "folder".to_string(), "location".to_string(), "uri".to_string()]);
    map.insert("file".to_string(), vec!["document".to_string(), "doc".to_string(), "source_file".to_string(), "rs_file".to_string(), "md_file".to_string(), "toml_file".to_string()]);
    map.insert("read".to_string(), vec!["load".to_string(), "open".to_string(), "fetch".to_string(), "get_content".to_string()]);
    map.insert("write".to_string(), vec!["save".to_string(), "output".to_string(), "store".to_string(), "create".to_string()]);
    map.insert("iterate".to_string(), vec!["loop".to_string(), "for_each".to_string(), "walk".to_string(), "traverse".to_string(), "enumerate".to_string()]);
    map.insert("match".to_string(), vec!["compare".to_string(), "equal".to_string(), "similar".to_string(), "correspond".to_string(), "align".to_string()]);
    map.insert("score".to_string(), vec!["similarity".to_string(), "metric".to_string(), "value".to_string(), "rating".to_string()]);
    map.insert("token".to_string(), vec!["word".to_string(), "lexeme".to_string(), "unit".to_string(), "element".to_string()]);
    map.insert("normalize".to_string(), vec!["standardize".to_string(), "clean".to_string(), "canonicalize".to_string(), "format".to_string()]);
    map.insert("ast".to_string(), vec!["syntax_tree".to_string(), "abstract_syntax_tree".to_string(), "tree".to_string(), "node".to_string(), "graph".to_string()]);
    map.insert("visitor".to_string(), vec!["walker".to_string(), "traverser".to_string(), "processor".to_string()]);
    map.insert("semantic".to_string(), vec!["meaning".to_string(), "context".to_string(), "intent".to_string()]);
    map.insert("index".to_string(), vec!["database".to_string(), "store".to_string(), "repository".to_string(), "cache".to_string(), "lookup".to_string()]);
    map.insert("chunk".to_string(), vec!["block".to_string(), "segment".to_string(), "part".to_string(), "fragment".to_string()]);

    // Rust-Specific Terms/Crates
    map.insert("syn".to_string(), vec!["parse_file".to_string(), "parse_str".to_string(), "Item".to_string(), "Expr".to_string(), "Stmt".to_string(), "FnArg".to_string(), "Type".to_string(), "Ident".to_string(), "Lit".to_string(), "Token".to_string()]);
    map.insert("quote".to_string(), vec!["quote!".to_string(), "to_tokens".to_string()]);
    map.insert("proc_macro2".to_string(), vec!["Span".to_string(), "TokenStream".to_string()]);
    map.insert("std::fs".to_string(), vec!["read_to_string".to_string(), "read_dir".to_string(), "metadata".to_string()]);
    map.insert("std::path".to_string(), vec!["Path".to_string(), "PathBuf".to_string(), "extension".to_string(), "file_name".to_string()]);
    map.insert("std::collections".to_string(), vec!["HashSet".to_string(), "HashMap".to_string()]);
    map.insert("walkdir".to_string(), vec!["WalkDir".to_string(), "into_iter".to_string(), "filter_entry".to_string(), "is_file".to_string(), "is_dir".to_string()]);
    map.insert("Result".to_string(), vec!["Ok".to_string(), "Err".to_string(), "map_err".to_string(), "?".to_string()]);
    map.insert("Option".to_string(), vec!["Some".to_string(), "None".to_string(), "map_or".to_string()]);
    map.insert("String".to_string(), vec!["to_string".to_string(), "to_lowercase".to_string(), "split".to_string(), "contains".to_string()]);
    map.insert("Vec".to_string(), vec!["push".to_string(), "iter".to_string(), "enumerate".to_string(), "sort_by".to_string()]);
    map.insert("HashSet".to_string(), vec!["insert".to_string(), "intersection".to_string(), "union".to_string()]);
    map.insert("HashMap".to_string(), vec!["insert".to_string(), "get".to_string()]);
    map.insert("derive".to_string(), vec!["Debug".to_string(), "Clone".to_string(), "PartialEq".to_string(), "Eq".to_string(), "Hash".to_string()]);
    map.insert("pub".to_string(), vec!["public".to_string(), "export".to_string()]);
    map.insert("mod".to_string(), vec!["module".to_string()]);
    map.insert("crate".to_string(), vec!["package".to_string(), "lib".to_string()]);
    map.insert("ffi".to_string(), vec!["foreign_function_interface".to_string(), "c_interface".to_string(), "bindings".to_string()]);
    map.insert("bootstrap".to_string(), vec!["init".to_string(), "setup".to_string(), "start".to_string(), "configure".to_string()]);
    map.insert("zos".to_string(), vec!["zero_one_system".to_string(), "ontology_system".to_string()]);
    return map;
}

// Function to extract keywords from comments
fn extract_keywords_from_comments(code: &str) -> HashSet<String> {
    let mut keywords = HashSet::new();
    let re = Regex::new(r"//\s*KEYWORDS:\s*(.*)").unwrap();
    for line in code.lines() {
        if let Some(captures) = re.captures(line) {
            if let Some(kw_str) = captures.get(1) {
                for kw in kw_str.as_str().split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()) {
                    keywords.insert(kw);
                }
            }
        }
    }
    keywords
}

pub fn find_duplicate_code(suggested_code: &str, search_path: &Path) -> Result<Vec<CodeMatch>, CodeAnalysisError> {
    let mut suggested_tokens = tokenize(suggested_code);
    let synonyms = get_synonyms();
    let mut matches: Vec<CodeMatch> = Vec::new();

    // Add keywords from suggested_code comments
    let suggested_comment_keywords = extract_keywords_from_comments(suggested_code);
    suggested_tokens.extend(suggested_comment_keywords);

    // Extend suggested tokens with synonyms
    let mut extended_suggested_tokens = suggested_tokens.clone();
    for token in &suggested_tokens {
        if let Some(syns) = synonyms.get(token) {
            for syn in syns {
                extended_suggested_tokens.insert(syn.clone());
            }
        }
    }

    for entry in walkdir::WalkDir::new(search_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_content = fs::read_to_string(path)?;
            let mut file_tokens = tokenize(&file_content);

            // Add keywords from file comments
            let file_comment_keywords = extract_keywords_from_comments(&file_content);
            file_tokens.extend(file_comment_keywords);

            let mut extended_file_tokens = file_tokens.clone();
            for token in &file_tokens {
                if let Some(syns) = synonyms.get(token) {
                    for syn in syns {
                        extended_file_tokens.insert(syn.clone());
                    }
                }
            }

            // Calculate Jaccard similarity based on extended token sets
            let intersection = extended_suggested_tokens.intersection(&extended_file_tokens).count();
            let union = extended_suggested_tokens.union(&extended_file_tokens).count();

            let similarity_score = if union == 0 {
                0.0
            } else {
                intersection as f64 / union as f64
            };

            // Define a threshold for similarity
            if similarity_score > 0.1 { // Adjust threshold as needed
                // For now, we'll return the whole file content as the snippet
                // In a more advanced version, we'd find the specific similar block
                matches.push(CodeMatch {
                    file_path: path.to_path_buf(),
                    start_line: 1, // Placeholder
                    end_line: file_content.lines().count(), // Placeholder
                    code_snippet: file_content,
                    similarity_score,
                });
            }
        }
    }

    // Sort matches by similarity score in descending order
    matches.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap_or(std::cmp::Ordering::Equal));

    Ok(matches)
}
