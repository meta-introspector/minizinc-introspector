use git2::{Repository, Oid, Commit, DiffOptions, Diff, Tree};
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use chrono::{DateTime, Utc, TimeZone};
use walkdir::WalkDir;
use std::env;

const CRQ_FILE_PATTERN: &str = "crq_*.md";
const CRQ_HISTORY_SECTION_START: &str = "## Commit History";

lazy_static! {
    static ref COMMIT_ENTRY_REGEX: Regex = Regex::new(
        r"(?sU)\*\*Commit:\*\* `([0-9a-f]{40})`\n\*\*Subject:\*\* `(.*?)`\n\*\*Description:\*\*([\s\S]*?)(?=\n\*\*Commit:\*\*|\Z)"
    ).unwrap();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct CommitEntry {
    hash: String,
    subject: String,
    description: String,
    author_date: DateTime<Utc>,
}

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
        println!("No CRQ files found matching pattern '{}'.", CRQ_FILE_PATTERN);
        return Ok(());
    }
    println!("Found CRQ files: {:?}", crq_files);

    for crq_path in crq_files {
        println!("Processing CRQ file: {:?}", crq_path);
        process_crq_file(&repo, &crq_path, dry_run)?;
    }

    Ok(())
}

fn find_crq_files(repo_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut crq_files = Vec::new();
    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(Result::ok) // Filter out errors
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.starts_with("crq_") && file_name.ends_with(".md") {
                    crq_files.push(path.to_path_buf());
                }
            }
        }
    }
    Ok(crq_files)
}

fn process_crq_file(repo: &Repository, crq_path: &Path, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let repo_workdir = repo.workdir().ok_or("Repository workdir not found")?;
    let relative_crq_path = crq_path.strip_prefix(repo_workdir)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?; // Sort by time, oldest first
    revwalk.push_head()?;

    let mut relevant_commits = Vec::new();

    for oid in revwalk.filter_map(Result::ok) {
        let commit = repo.find_commit(oid)?;
        let tree = commit.tree()?;

        let mut modified = false;
        if let Some(parent) = commit.parent(0)? {
            let parent_tree = parent.tree()?;
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
            diff.deltas().for_each(|delta| {
                if let Some(path) = delta.new_file().path() {
                    if path == relative_crq_path {
                        modified = true;
                    }
                }
            });
        } else {
            // Initial commit, check if the file exists in this commit's tree
            if tree.get_path(relative_crq_path).is_ok() {
                modified = true;
            }
        }

        if modified {
            let commit_time = commit.time();
            let author_date = Utc.timestamp_opt(commit_time.seconds(), 0)
                .single().ok_or("Invalid commit timestamp")?;

            relevant_commits.push(CommitEntry {
                hash: commit.id().to_string(),
                subject: commit.summary().unwrap_or("").to_string(),
                description: commit.body().unwrap_or("").to_string(),
                author_date,
            });
        }
    }

    // Sort by author_date to ensure chronological order for display
    relevant_commits.sort_by(|a, b| a.author_date.cmp(&b.author_date));

    let crq_content = fs::read_to_string(crq_path)?;
    let (pre_history_content, mut existing_commit_entries) = extract_existing_history(repo, &crq_content)?;

    let mut new_entries_to_add = Vec::new();
    for commit_entry in relevant_commits {
        // Check if this commit hash already exists in the extracted history
        if !existing_commit_entries.iter().any(|e| e.hash == commit_entry.hash) {
            let description = if commit_entry.description.is_empty() {
                // If the commit body is empty, generate a summary from the diff
                let commit = find_commit_from_oid(repo, &commit_entry.hash)?;

                // Check if this commit created the file
                let is_creation_commit = commit.parent_count() == 0 || {
                    let parent_tree = commit.parent(0)?.tree()?;
                    parent_tree.get_path(relative_crq_path).is_err()
                };

                if is_creation_commit {
                    "This commit created the CRQ file. The file's initial content serves as its primary description.".to_string()
                } else {
                    get_commit_diff_summary(repo, commit, relative_crq_path)?
                }
            } else {
                commit_entry.description.clone()
            };

            new_entries_to_add.push(CommitEntry {
                hash: commit_entry.hash,
                subject: commit_entry.subject,
                description,
                author_date: commit_entry.author_date,
            });
        }
    }

    // Combine existing and new entries, sort them, and format for writing
    existing_commit_entries.extend(new_entries_to_add);
    existing_commit_entries.sort_by(|a, b| a.author_date.cmp(&b.author_date));

    let mut formatted_history = String::new();
    formatted_history.push_str(CRQ_HISTORY_SECTION_START);
    for entry in existing_commit_entries {
        formatted_history.push_str(&format!(
            "\n\n**Commit:** `{}`\n**Subject:** `{}`\n**Description:**\n{}",
            entry.hash, entry.subject, entry.description
        ));
    }

    let final_content = format!("{}{}", pre_history_content, formatted_history);

    if dry_run {
        println!("\n--- DRY-RUN: Changes for {:?} ---", crq_path);
        println!("{}", final_content);
        println!("--- END DRY-RUN: Changes for {:?} ---\n", crq_path);
    } else {
        fs::write(crq_path, final_content)?;
        println!("Updated CRQ file: {:?}", crq_path);
    }

    Ok(())
}

fn extract_existing_history(repo: &Repository, crq_content: &str) -> Result<(String, Vec<CommitEntry>), Box<dyn std::error::Error>> {
    if let Some(history_start_index) = crq_content.find(CRQ_HISTORY_SECTION_START) {
        let (pre_history, history_section) = crq_content.split_at(history_start_index);
        let mut existing_entries = Vec::new();

        for cap in COMMIT_ENTRY_REGEX.captures_iter(history_section) {
            let hash = cap[1].to_string();
            let subject = cap[2].to_string();
            let description = cap[3].trim().to_string();

            // Fetch author_date from the repository for the commit hash
            let commit_obj = repo.find_object(Oid::from_str(&hash)?)?;
            let commit = commit_obj.as_commit().ok_or("Not a commit object")?;
            let commit_time = commit.time();
            let author_date = Utc.timestamp_opt(commit_time.seconds(), 0)
                .single().ok_or("Invalid commit timestamp")?;

            existing_entries.push(CommitEntry {
                hash,
                subject,
                description,
                author_date,
            });
        }
        Ok((pre_history.to_string(), existing_entries))
    } else {
        Ok((crq_content.to_string(), Vec::new()))
    }
}

fn get_commit_diff_summary(repo: &Repository, commit: &Commit, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let parent_tree = commit.parent(0).and_then(|p| p.tree().ok());
    let current_tree = commit.tree()?;

    let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), None)?;
    
    let mut diff_summary = String::new();
    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        // Filter lines to only include those relevant to the specific file
        let path_matches = if let Some(path) = delta.new_file().path() {
            path == file_path
        } else if let Some(path) = delta.old_file().path() {
            path == file_path
        } else {
            false
        };

        if path_matches {
            diff_summary.push(line.origin());
            diff_summary.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
        }
        true
    })?;

    if diff_summary.is_empty() {
        Ok("No specific diff content for this file in this commit (might be creation or deletion, or only metadata changes). If this is a creation, the file content is the description of the CRQ.".to_string())
    } else {
        Ok(format!("Changes to this file in this commit:\n```diff\n{}\n```", diff_summary))
    }
}

fn find_commit_from_oid<'a>(repo: &'a Repository, oid_str: &str) -> Result<Commit<'a>, Box<dyn std::error::Error>> {
    let oid = Oid::from_str(oid_str)?;
    let commit_obj = repo.find_object(oid, None)?;
    let commit = commit_obj.as_commit().ok_or("Not a commit object")?;
    Ok(commit)
}
