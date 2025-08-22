use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::Path;
#[cfg(test)]
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoemReportEntry {
    pub file_path: String,
    pub status: String,
    pub matched_lines_by_regex: Option<HashMap<String, HashMap<String, usize>>>,
    pub error_message: Option<String>,
    pub extracted_words_count: Option<usize>,
    pub dry_run_changes_applied: bool,
    pub unmatched_lines: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoemReport {
    pub files: Vec<PoemReportEntry>,
}

pub fn generate_and_save_report(
    report_entries: Vec<PoemReportEntry>,
    current_dir: &Path,
) -> Result<()> {
    let report = PoemReport { files: report_entries };
    let report_yaml = serde_yaml::to_string(&report)?;
    let report_path = current_dir.join("poem_processing_report.yaml");
    std::fs::write(&report_path, report_yaml)?;

    println!("Report generated at: {:?}", report_path);

    Ok(())
}
