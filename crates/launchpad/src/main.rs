use std::env;
use std::process::{Command, Stdio};
mod orchestrator;
mod narrator;
mod dum_wrappers;
mod launchpad_main;

#[tokio::main]
async fn main() -> Result<(), String> {
    launchpad_main::run_launchpad().await
}
