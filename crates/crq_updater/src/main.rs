use git2::Repository;
use std::path::Path;
use std::env;

mod functions;

use functions::find_crq_files::find_crq_files;
use functions::process_crq_file::process_crq_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let dry_run = args.iter().any(|arg| arg == "--dry-run");

    let repo_path = Path::new("."); // Current directory
    let repo = Repository::open(repo_path)?;

    println!("Opened Git repository at: {:?}", repo.path());
    if dry_run {
        println!("Running in DRY-RUN mode. No files will be modified.");
    }

    let crq_files = find_crq_files(repo_path)?;
    if crq_files.is_empty() {
        println!("No CRQ files found matching pattern 'crq_*.md'.");
        return Ok(());
    }
    println!("Found CRQ files: {:?}", crq_files);

    for crq_path in crq_files {
        println!("Processing CRQ file: {:?}", crq_path);
        process_crq_file(&repo, &crq_path, dry_run)?;
    }

    Ok(())
}