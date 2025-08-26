use crate::orchestrator;
use crate::stages::Stage;
use git2::Repository; // Not directly used by tmux_stage, but required by Stage trait

pub struct TmuxStage;

impl Stage for TmuxStage {
    fn name(&self) -> &str {
        "tmux"
    }

    async fn run(&self, _repo: &Repository, args: &[&str]) -> Result<(), String> {
        orchestrator::run_tmux_command(args).await
    }
}
