use crate::orchestrator;
use crate::stages::Stage;
use git2::Repository;
use std::pin::Pin;
use std::future::Future;

pub struct TmuxStage;

impl Stage for TmuxStage {
    fn name(&self) -> &str {
        "tmux"
    }

    fn run(&self, _repo: &Repository, args: &[&str]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let args_vec: Vec<String> = args.iter().map(|&s| s.to_string()).collect();
        Box::pin(async move {
            orchestrator::run_tmux_command(&args_vec.iter().map(|s| s.as_str()).collect::<Vec<&str>>()).await
        })
    }
}