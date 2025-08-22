use std::path::Path;
use anyhow::Result;
use crate::functions::report_generator::PoemReportEntry;

pub fn print_detailed_regex_report(report_entries: & [PoemReportEntry], current_dir: &Path) -> Result<()> {
    let mut detailed_report_content = String::new();
    detailed_report_content.push_str("---\n");
    for entry in report_entries {
        detailed_report_content.push_str(&format!("\nFile: {}\n", entry.file_path));
        if let Some(matched_lines) = &entry.matched_lines_by_regex {
            detailed_report_content.push_str("  Matched Lines by Regex:\n");
            for (regex_name, lines_map) in matched_lines {
                detailed_report_content.push_str(&format!("    Regex: {}\n", regex_name));
                for (line_content, count) in lines_map {
                    detailed_report_content.push_str(&format!("      - \"{}\": {}\n", line_content, count));
                }
            }
        } else {
            detailed_report_content.push_str("  No regex matches recorded for this file.\n");
        }

        if let Some(unmatched_lines) = &entry.unmatched_lines {
            detailed_report_content.push_str("  Unmatched Lines:\n");
            if unmatched_lines.is_empty() {
                detailed_report_content.push_str("    (None)\n");
            } else {
                for line in unmatched_lines {
                    detailed_report_content.push_str(&format!("    - \"{}\"\n", line));
                }
            }
        }
    }
    let detailed_report_path = current_dir.join("regex_match_details.txt");
    std::fs::write(&detailed_report_path, detailed_report_content)?;
    println!("Detailed regex match report generated at: {:?}\n", detailed_report_path);
    Ok(())
}