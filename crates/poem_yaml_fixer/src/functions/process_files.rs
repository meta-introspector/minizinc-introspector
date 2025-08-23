


use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::functions::types::{FixedFrontMatter, PoemFunctionRegistry};
use crate::RegexConfig;
use crate::manual_parser::manual_parse_poem_file;
use crate::process_file;
use crate::functions::report_generator::PoemReportEntry;
use crate::functions::report_printer::print_detailed_regex_report;
use crate::functions::process_unmatched_lines_for_grex::process_unmatched_lines_for_grex;
use std::io::Write;

pub fn process_files(
    cli_file: &Option<PathBuf>,
    cli_manual_parse: bool,
    cli_debug: bool,
    cli_dry_run: bool,
    cli_report: bool,
    cli_generate_grex_regex: bool,
    poems_dir: &PathBuf,
    current_dir: &PathBuf,
    regex_config: &RegexConfig,
    function_registry: &PoemFunctionRegistry,
    log_dir: &PathBuf, // Change from &Option<PathBuf> to &PathBuf
) -> Result<()> {
    let mut report_entries: Vec<PoemReportEntry> = Vec::new();

    if let Some(file_path) = cli_file {
        if cli_manual_parse {
            let log_output = format!("Using manual parser for: {:?}\n---\nManual Parse Result (Fixed Front Matter):\n{}\n---", file_path, serde_yaml::to_string(&FixedFrontMatter::default())?);
            let file_name = file_path.file_name().unwrap_or_default().to_string_lossy().replace(".md", ".log");
            let log_file_path = log_dir.join(file_name);
            let mut file = std::fs::File::create(&log_file_path)?;
            file.write_all(log_output.as_bytes())?;
        } else {
            process_file(&file_path, regex_config, function_registry, &mut report_entries, cli_debug, cli_dry_run, log_dir)?;
        }
    } else {
        for entry in WalkDir::new(poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                if path.file_name().is_some_and(|name| name.to_str().unwrap_or("").ends_with(".archeology.md")) {
                    continue;
                }
                if cli_manual_parse {
                    let log_output = format!("Using manual parser for: {:?}\n---\nManual Parse Result (Fixed Front Matter):\n{}\n---", path, serde_yaml::to_string(&FixedFrontMatter::default())?);
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy().replace(".md", ".log");
                    let log_file_path = log_dir.join(file_name);
                    let mut file = std::fs::File::create(&log_file_path)?;
                    file.write_all(log_output.as_bytes())?;
                } else {
                    process_file(path, regex_config, function_registry, &mut report_entries, cli_debug, cli_dry_run, log_dir)?;
                }
            }
        }
    }

    // After processing all files, conditionally generate the report
    if cli_report {
        print_detailed_regex_report(&report_entries, current_dir)?;

        // Process unmatched lines for grex per poem, if enabled
        if cli_generate_grex_regex {
            for entry in &report_entries {
                if let Some(unmatched) = &entry.unmatched_lines {
                    if !unmatched.is_empty() {
                        // Need to pass the actual file_path for the poem
                        // The file_path in PoemReportEntry is a String, convert to Path
                        let poem_path = PathBuf::from(&entry.file_path);
                        process_unmatched_lines_for_grex(unmatched, &poem_path, current_dir, log_dir)?;
                    }
                }
            }
        }

        crate::functions::report_generator::generate_and_save_report(report_entries, current_dir)?;
    }

    Ok(())
}

