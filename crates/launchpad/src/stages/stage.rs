use git2::Repository; // Assuming stages might need repo access
use std::path::Path;

pub trait Stage {
    fn name(&self) -> &str;
    async fn run(&self, repo: &Repository, args: &[&str]) -> Result<(), String>;
}
