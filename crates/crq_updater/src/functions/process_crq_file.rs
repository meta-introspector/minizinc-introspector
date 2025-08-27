use git2::{Repository, Oid, Commit, Tree};
use std::path::{Path, PathBuf};
use std::fs;
use chrono::{DateTime, Utc, TimeZone};

use super::extract_existing_history::extract_existing_history;
use super::get_commit_diff_summary::get_commit_diff_summary;
use super::find_commit_from_oid::find_commit_from_oid;

const CRQ_HISTORY_SECTION_START: &str = "## Commit History";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CommitEntry {
    pub hash: String,
    pub subject: String,
    pub description: String,
    pub author_date: DateTime<Utc>,
}

pub fn process_crq_file(repo: &Repository, crq_path: &Path, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let repo_workdir = repo.workdir().ok_or("Repository workdir not found")?;
    let relative_crq_path = crq_path.strip_prefix(repo_workdir)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)??; // Sort by time, oldest first
    revwalk.push_head()?;

    let mut relevant_commits = Vec::new();

    for oid in revwalk.filter_map(Result::ok) {
        let commit = repo.find_commit(oid)?;
        let tree = commit.tree()?;

        let mut modified = false;
        if let Some(parent) = commit.parent(0)? {
            let parent_tree = parent.parent(0)?.tree()?;
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
        println!("--- END DRY-RUN: Changes for {:?} ---\
", crq_path);
    } else {
        fs::write(crq_path, final_content)?;
        println!("Updated CRQ file: {:?}", crq_path);
    }

    Ok(())
}
