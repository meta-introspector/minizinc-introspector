// crates/constant_analyzer/src/constants.rs


pub const DEFAULT_REPORTS_DIR: &str = "./reports";
pub const DEFAULT_REPORT_FILENAME: &str = "constant_analysis_report.txt";
pub const NAMING_SOLVER_MODEL_PATH: &str = "naming_solver.mzn";
use std::path::PathBuf;
use std::env;

pub fn get_project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let mut path = current_dir.as_path();

    loop {
        // Check for .git directory
        if path.join(".git").is_dir() {
            return Ok(path.to_path_buf());
        }
        // Check for Cargo.toml (for non-git Rust projects or sub-crates)
        if path.join("Cargo.toml").is_file() {
            // If it's a Cargo.toml, check if it's a workspace root
            let cargo_toml_content = std::fs::read_to_string(path.join("Cargo.toml"))?;
            if cargo_toml_content.contains("[workspace]") {
                return Ok(path.to_path_buf());
            }
        }

        match path.parent() {
            Some(p) => path = p,
            None => return Err("Project root not found. Neither .git nor Cargo.toml (with [workspace]) found in parent directories.".into()),
        }
    }
}