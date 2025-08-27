use git2::{Repository, Oid, Commit};

pub fn find_commit_from_oid<'a>(repo: &'a Repository, oid_str: &str) -> Result<Commit<'a>, Box<dyn std::error::Error>> {
    let oid = Oid::from_str(oid_str)?;
    let commit_obj = repo.find_object(oid, None)?;
    let commit = commit_obj.as_commit().ok_or("Not a commit object")?;
    Ok(commit.to_owned())
}
