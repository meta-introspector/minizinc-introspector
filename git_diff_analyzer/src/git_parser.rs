use git2::{Repository, Commit, Diff, DiffOptions, Oid};
use anyhow::{Result, anyhow};
use std::path::Path;

pub fn open_repository<P: AsRef<Path>>(path: P) -> Result<Repository> {
    Repository::open(path).map_err(|e| anyhow!("Failed to open repository: {}", e))
}

pub fn get_last_n_commits(repo: &Repository, n: usize) -> Result<Vec<Commit>> {
    let head = repo.head()?.resolve()?;
    let head_commit = head.peel_to_commit()?;

    let mut commits = Vec::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push(head_commit.id())?;
    revwalk.set_sorting(git2::Sort::TIME)?; // Sort by commit time

    for (i, oid) in revwalk.into_iter().enumerate() {
        if i >= n {
            break;
        }
        let commit = repo.find_commit(oid?)?;
        commits.push(commit);
    }
    Ok(commits)
}

pub fn get_commit_diff<'a>(repo: &'a Repository, commit: &Commit) -> Result<Diff<'a>> {
    let parent = if commit.parent_count() > 0 {
        Some(commit.parent(0)?)
    } else {
        None
    };

    let diff = repo.diff_tree_to_tree(
        parent.as_ref().map(|p| p.tree().ok()).flatten().as_ref(),
        Some(&commit.tree()?), 
        None,
    )?;
    Ok(diff)
}

pub struct DiffLine {
    pub content: String,
    pub is_added: bool,
}

pub fn extract_diff_lines(diff: &Diff) -> Result<Vec<DiffLine>> {
    let mut lines = Vec::new();
    diff.print(git2::DiffFormat::Patch, |delta, hunk, line| {
        let content = String::from_utf8_lossy(line.content()).into_owned();
        match line.origin() {
            '+' => lines.push(DiffLine { content, is_added: true }),
            '-' => lines.push(DiffLine { content, is_added: false }),
            _ => {},
        }
        true
    })?;
    Ok(lines)
}
