use std::path::Path;
use anyhow::Result;
use crate::functions::report_generator::PoemReportEntry;
use serde_yaml;

pub fn print_detailed_regex_report(report_entries: & [PoemReportEntry], current_dir: &Path) -> Result<()> {
    let detailed_report_content = serde_yaml::to_string(report_entries)?;
    let detailed_report_path = current_dir.join("regex_match_details.yaml"); // Change extension to .yaml
    std::fs::write(&detailed_report_path, detailed_report_content)?;
    println!("Detailed regex match report generated at: {:?}\n", detailed_report_path);
    Ok(())
}
