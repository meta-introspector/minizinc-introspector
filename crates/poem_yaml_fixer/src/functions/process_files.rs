use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::functions::types::{FixedFrontMatter, PoemFunctionRegistry};
use crate::RegexConfig;
use crate::manual_parser::manual_parse_poem_file;
use crate::process_file;
use crate::functions::report_generator::PoemReportEntry;
use crate::functions::report_printer::print_detailed_regex_report;
use crate::functions::report_processing::process_poems_for_report;

pub fn process_files(
    cli_file: &Option<PathBuf>,
    cli_manual_parse: bool,
    cli_debug: bool,
    cli_dry_run: bool,
    cli_report: bool,
    poems_dir: &PathBuf,
    current_dir: &PathBuf,
    regex_config: &RegexConfig,
    function_registry: &PoemFunctionRegistry,
) -> Result<()> {
    let mut report_entries: Vec<PoemReportEntry> = Vec::new();

    if let Some(file_path) = cli_file {
        if cli_manual_parse {
            println!("Using manual parser for: {:?}", file_path);
            let content = std::fs::read_to_string(&file_path)?;
            let mut fixed_fm = FixedFrontMatter::default();
            manual_parse_poem_file(&content, &mut fixed_fm)?;
            println!("--- Manual Parse Result (Fixed Front Matter) ---");
            println!("{}", serde_yaml::to_string(&fixed_fm)?);
            println!("------------------------------------------------");
        } else {
            process_file(&file_path, regex_config, function_registry, &mut report_entries, cli_debug, cli_dry_run)?;
        }
    } else {
        for entry in WalkDir::new(poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                if path.file_name().is_some_and(|name| name.to_str().unwrap_or("").ends_with(".archeology.md")) {
                    continue;
                }
                if cli_manual_parse {
                    println!("Using manual parser for: {:?}\n", path);
                    let content = std::fs::read_to_string(&path)?;
                    let mut fixed_fm = FixedFrontMatter::default();
                    manual_parse_poem_file(&content, &mut fixed_fm)?;
                    println!("--- Manual Parse Result (Fixed Front Matter) ---
");
                    println!("{}\n", serde_yaml::to_string(&fixed_fm)?);
                    println!("------------------------------------------------\n");
                } else {
                    process_file(path, regex_config, function_registry, &mut report_entries, cli_debug, cli_dry_run)?;
                }
            }
        }
    }

    // After processing all files, conditionally generate the report
    if cli_report {
        print_detailed_regex_report(&report_entries, current_dir)?;
        crate::functions::report_generator::generate_and_save_report(report_entries, current_dir)?;
    }

    Ok(())
}