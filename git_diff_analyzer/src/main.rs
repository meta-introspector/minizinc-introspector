use clap::Parser;
use anyhow::Result;
use std::collections::HashMap;

mod git_parser;
mod text_processor;
mod crq_loader;
mod vector_utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Git repository to analyze
    #[arg(long, default_value = ".")]
    repo_path: String,

    /// Number of commits to analyze
    #[arg(long, default_value_t = 20)]
    num_commits: usize,

    /// Path to the directory containing CRQ documents
    #[arg(long, default_value = "./docs/crqs")]
    crq_dir: String,

    /// Number of top similar CRQs to display for each commit
    #[arg(long, default_value_t = 3)]
    top_n: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Analyzing Git repository: {}", args.repo_path);
    println!("Number of commits to analyze: {}", args.num_commits);
    println!("CRQ directory: {}", args.crq_dir);

    let repo = git_parser::open_repository(&args.repo_path)?;
    let commits = git_parser::get_last_n_commits(&repo, args.num_commits)?;

    // Load CRQ documents
    println!("\nLoading CRQ documents from: {}", args.crq_dir);
    let crq_documents = crq_loader::load_crq_documents(&args.crq_dir)?;
    println!("Loaded {} CRQ documents.", crq_documents.len());

    // Collect all bag-of-words for global vocabulary creation
    let mut all_bovs: Vec<&HashMap<String, usize>> = Vec::new();
    let mut commit_bovs: Vec<HashMap<String, usize>> = Vec::new();

    for commit in &commits {
        let diff = git_parser::get_commit_diff(&repo, commit)?;
        let diff_lines = git_parser::extract_diff_lines(&diff)?;
        
        let mut added_text = String::new();
        for line in diff_lines {
            if line.is_added {
                added_text.push_str(&line.content);
                added_text.push_str(" ");
            }
        }

        let cleaned_words = text_processor::clean_and_tokenize(&added_text);
        let filtered_words = text_processor::filter_stop_words(cleaned_words);
        let commit_bow = text_processor::generate_bag_of_words(filtered_words);
        commit_bovs.push(commit_bow);
    }

    for crq_doc in &crq_documents {
        all_bovs.push(&crq_doc.bag_of_words);
    }
    for commit_bow in &commit_bovs {
        all_bovs.push(commit_bow);
    }

    let vocabulary = vector_utils::create_vocabulary(&all_bovs);
    println!("Global vocabulary size: {}", vocabulary.len());

    println!("\nAnalyzing commits and finding similar CRQs:");
    for (i, commit) in commits.iter().enumerate() {
        println!("  Commit: {} - {}", commit.id(), commit.summary().unwrap_or("No summary"));
        let commit_vec = vector_utils::bov_to_vector(&commit_bovs[i], &vocabulary);

        let mut similarities: Vec<(f64, &CrqDocument)> = Vec::new();
        for crq_doc in &crq_documents {
            let crq_vec = vector_utils::bov_to_vector(&crq_doc.bag_of_words, &vocabulary);
            let similarity = vector_utils::cosine_similarity(commit_vec.view(), crq_vec.view());
            similarities.push((similarity, crq_doc));
        }

        similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        println!("    Top {} similar CRQs:", args.top_n);
        for (sim, crq_doc) in similarities.iter().take(args.top_n) {
            println!("      Similarity: {{:.4}} - CRQ: {{:?}}", sim, crq_doc.path);
        }
    }

    Ok(())
}