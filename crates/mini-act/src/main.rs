mod workflow;
mod runner;
mod git_actions;
mod gemini_context_args;

use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};
use gemini_utils::gemini_eprintln;
use kantspel_macros::kantspel_transform;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates Gemini CLI calls with MiniZinc-sized contexts
    GeminiContext { #[clap(flatten)] args: gemini_context_args::GeminiContextArgs },
}

#[kantspel_transform]
mod app_logic {
    use super::*;

    pub fn run_app() {
        let cli = Cli::parse();

        match cli.command {
            Some(Commands::GeminiContext { args }) => {
                eprintln!("GeminiContext subcommand called with args: {:?}", args);
            },
            None => {
                // Original workflow execution logic
                let workflows_dir = Path::new(".github/workflows");

                if !workflows_dir.exists() || !workflows_dir.is_dir() {
                    gemini_eprintln!("Error: ::brick:: .github/workflows directory not found or is not a directory.");
                    return;
                }

                gemini_eprintln!("Running all workflows in ::inspect::");

                for entry in fs::read_dir(workflows_dir).expect("Failed to read workflows directory") {
                    let entry = entry.expect("Failed to read directory entry");
                    let path = entry.path();

                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if extension == "yml" || extension == "yaml" {
                                gemini_eprintln!("::newline::::brick:: Running workflow: ::inspect:: ::brick::");
                                let workflow_content = match fs::read_to_string(&path) {
                                    Ok(content) => content,
                                    Err(_e) => {
                                        gemini_eprintln!("Error reading workflow file ::inspect:: ::brick:: ::variable::");
                                        continue;
                                    }
                                };

                                let workflow: workflow::Workflow = match serde_yaml::from_str(&workflow_content) {
                                    Ok(wf) => wf,
                                    Err(_e) => {
                                        gemini_eprintln!("Error parsing workflow file ::inspect:: ::brick:: ::variable::");
                                        continue;
                                    }
                                };

                                runner::run_workflow(&workflow);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    app_logic::run_app();
}
