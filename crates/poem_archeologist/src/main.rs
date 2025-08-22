use anyhow::Result;
use git2::{Repository, Oid};
use std::collections::HashSet;
use std::fs;
use std::path::{Path,
		//PathBuf
};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let repo_path = ".";
    let repo = Repository::open(repo_path)?;
    println!("Opened repository at: {:?}", repo.path());

    let poems_dir = "docs/poems";

    for entry in WalkDir::new(poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            println!("\nProcessing file: {:?}", path);
            let history = get_file_history(&repo, path)?;
            if !history.is_empty() {
                let current_content = fs::read_to_string(path)?;
                let mut archeology_content = String::new();
                let mut current_words = get_words(&current_content);

                for (commit_id, old_content) in history.iter().rev() {
                    let old_words = get_words(old_content);
                    if !old_words.is_subset(&current_words) {
                        println!("  Appending content from commit: {}", commit_id);
                        archeology_content.push_str("\n\n---\n\n");
                        archeology_content.push_str(&format!("## Lost Revision from commit {}\n\n", commit_id));
                        archeology_content.push_str(old_content);
                        current_words.extend(old_words);
                    }
                }

                if !archeology_content.is_empty() {
                    let archeology_file_path = path.with_extension("md.archeology.md");
                    fs::write(&archeology_file_path, archeology_content)?;
                    println!("  Created archeology file: {:?}", archeology_file_path);
                }
            }
        }
    }

    Ok(())
}

fn get_file_history(repo: &Repository, path: &Path) -> Result<Vec<(Oid, String)>> {
    let mut history = Vec::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let commit_oid = oid?;
        let commit = repo.find_commit(commit_oid)?;
        let tree = commit.tree()?;

        if let Ok(entry) = tree.get_path(path) {
            if let Some(blob) = repo.find_blob(entry.id()).ok() {
                if let Ok(content) = std::str::from_utf8(blob.content()) {
                    history.push((commit_oid, content.to_string()));
                }
            }
        }
    }

    Ok(history)
}

fn get_words(content: &str) -> HashSet<String> {
    content
        .split_whitespace()
        .map(|s| s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect())
        .collect()
}
