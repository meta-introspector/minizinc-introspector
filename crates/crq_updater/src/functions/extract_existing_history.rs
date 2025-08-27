use std::collections::HashSet;
use git2::Repository;
use regex::Regex;
use lazy_static::lazy_static;
use chrono::{Utc, TimeZone};

use super::process_crq_file::CommitEntry;
use super::find_commit_from_oid::find_commit_from_oid;

const CRQ_HISTORY_SECTION_START: &str = "## Commit History";

lazy_static! {
    // Regex to match the start of a commit entry (hash and subject)
    static ref COMMIT_START_REGEX: Regex = Regex::new(
        r"\*\*Commit:\*\* `([0-9a-f]{40})`\n\*\*Subject:\*\* `(.*?)`\n\*\*Description:\*\*"
    ).unwrap();
}

pub fn extract_existing_history(repo: &Repository, crq_content: &str) -> Result<(String, Vec<CommitEntry>), Box<dyn std::error::Error>> {
    if let Some(history_start_index) = crq_content.find(CRQ_HISTORY_SECTION_START) {
        let (pre_history, history_section_full) = crq_content.split_at(history_start_index);
        let mut existing_entries = Vec::new();
        let mut seen_hashes = HashSet::new();

        eprintln!("History section: {}", history_section_full);

        let mut current_pos = 0;
        while let Some(captures) = COMMIT_START_REGEX.captures(&history_section_full[current_pos..]) {
            let hash = captures.get(1).unwrap().as_str().to_string();
            let subject = captures.get(2).unwrap().as_str().to_string();
            let commit_entry_start_offset = current_pos + captures.get(0).unwrap().start();
            let description_content_start_offset = current_pos + captures.get(0).unwrap().end();

            let remaining_history = &history_section_full[description_content_start_offset..];

            let next_commit_match = COMMIT_START_REGEX.find(remaining_history);

            let description_end_offset = if let Some(next_match) = next_commit_match {
                next_match.start()
            } else {
                remaining_history.len()
            };

            let description = remaining_history[..description_end_offset].trim().to_string();

            if seen_hashes.insert(hash.clone()) {
                eprintln!("Found existing entry: hash={}, subject={}", hash, subject);

                // Fetch author_date from the repository for the commit hash
                let commit = find_commit_from_oid(repo, &hash)?;
                let commit_time = commit.time();
                let author_date = Utc.timestamp_opt(commit_time.seconds(), 0)
                    .single().ok_or("Invalid commit timestamp")?;

                existing_entries.push(CommitEntry {
                    hash,
                    subject,
                    description,
                    author_date,
                });
            } else {
                eprintln!("Skipping duplicate existing entry: hash={}", hash);
            }

            current_pos = description_content_start_offset + description_end_offset;
        }
        eprintln!("Existing entries found: {:?}", existing_entries);
        Ok((pre_history.to_string(), existing_entries))
    } else {
        eprintln!("No history section found.");
        Ok((crq_content.to_string(), Vec::new()))
    }
}