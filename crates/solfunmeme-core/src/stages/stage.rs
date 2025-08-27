use git2::Repository;
use std::pin::Pin;
use std::future::Future;

pub trait Stage {
    fn name(&self) -> &str;
    // Change async fn to return a Pin<Box<dyn Future>>
    fn run(&self, repo: &Repository, args: &[&str]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
}