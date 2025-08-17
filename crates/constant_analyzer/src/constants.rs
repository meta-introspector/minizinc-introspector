// crates/constant_analyzer/src/constants.rs

pub const REPORT_HEADER: &str = "--- Constant Analysis Report ---\\n";
pub const CONSTANT_DETAIL_FORMAT: &str = "Constant: '{}'\\n  Declared in: {{:?}}\\n  Usage Count: {{}}\\n";
pub const STATUS_FLAGGED: &str = "  STATUS: FLAGGED (Used more than once)\\n";
pub const STATUS_OK: &str = "  STATUS: OK (Used once or not at all)\\n";
pub const PLAN_MOVE_CONSTANT: &str = "  PLAN: Consider moving this constant to a dedicated constants module (e.g., `constants.rs`, `config.rs`, or `types.rs`) to centralize its definition.\\n";
pub const NOTE_SINGLE_USE_NOT_DEDICATED: &str = "  NOTE: This constant is not in a dedicated constants file, but is only used once. Consider if it should be inlined or moved for future clarity.\\n";
pub const NOTE_ALREADY_DEDICATED: &str = "  NOTE: This constant is already in a dedicated constants file/directory.\\n";
pub const REPORT_SEPARATOR: &str = "--------------------------------------------------\\n";
pub const PHASE1_MESSAGE: &str = "Phase 1: Ingesting and parsing Rust files from {{:?}}";
pub const PROCESSING_FILE_MESSAGE: &str = "  Processing file: {{:?}}";
pub const PHASE2_MESSAGE: &str = "\\nPhase 2: Analyzing constant usages...";
pub const PHASE3_MESSAGE: &str = "\\nPhase 3: Generating Constant Analysis Report and Refactoring Plan...";
pub const WRITING_REPORT_MESSAGE: &str = "Writing report to: {{:?}}";
pub const ANALYSIS_COMPLETE_MESSAGE: &str = "\\nAnalysis complete. Report saved to {{:?}}";
pub const DEFAULT_REPORTS_DIR: &str = "./reports";
pub const DEFAULT_REPORT_FILENAME: &str = "constant_analysis_report.txt";
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

