use git2::{Repository, Commit};
use std::path::Path;

pub fn get_commit_diff_summary(repo: &Repository, commit: &Commit, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let parent_tree = commit.parent(0).ok().and_then(|p| p.tree().ok());
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
