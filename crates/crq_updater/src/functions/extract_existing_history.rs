use git2::{Repository, Oid};
use regex::Regex;
use lazy_static::lazy_static;
use chrono::{DateTime, Utc, TimeZone};

use super::process_crq_file::CommitEntry;
use super::find_commit_from_oid::find_commit_from_oid;

const CRQ_HISTORY_SECTION_START: &str = "## Commit History";

lazy_static! {
    static ref COMMIT_ENTRY_REGEX: Regex = Regex::new(
        r"(?sU)\*\*Commit:\*\* `([0-9a-f]{40})`\n\*\*Subject:\*\* `(.*?)`\n\*\*Description:\*\*([\s\S]*?)(?=\n\*\*Commit:\*\*|\Z)"
    ).unwrap();
}

pub fn extract_existing_history(repo: &Repository, crq_content: &str) -> Result<(String, Vec<CommitEntry>), Box<dyn std::error::Error>> {
    if let Some(history_start_index) = crq_content.find(CRQ_HISTORY_SECTION_START) {
        let (pre_history, history_section) = crq_content.split_at(history_start_index);
        let mut existing_entries = Vec::new();

        for cap in COMMIT_ENTRY_REGEX.captures_iter(history_section) {
            let hash = cap[1].to_string();
            let subject = cap[2].to_string();
            let description = cap[3].trim().to_string();

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
        }
        Ok((pre_history.to_string(), existing_entries))
    } else {
        Ok((crq_content.to_string(), Vec::new()))
    }
}
