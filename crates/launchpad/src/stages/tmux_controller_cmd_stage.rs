use crate::orchestrator;
use crate::stages::Stage;
use git2::Repository;
use std::pin::Pin;
use std::future::Future;

pub struct TmuxControllerCmdStage;

impl Stage for TmuxControllerCmdStage {
    fn name(&self) -> &str {
        "tmux-controller-cmd"
    }

    fn run(&self, _repo: &Repository, args: &[&str]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let args_vec: Vec<String> = args.iter().map(|&s| s.to_string()).collect();
        Box::pin(async move {
            // Execute cargo run --package tmux_controller -- <subcommand> [args...]
            let mut cmd_args = vec!["run", "--package", "tmux_controller", "--"];
            cmd_args.extend(args_vec.iter().map(|s| s.as_str()));

            orchestrator::run_command("cargo", &cmd_args, None).await
        })
    }
}
