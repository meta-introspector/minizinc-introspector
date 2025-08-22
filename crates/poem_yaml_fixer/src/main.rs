use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

use crate::functions::process_single_poem_file_for_report::process_single_poem_file_for_report;

poem_macros::poem_header!();

mod functions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional path to a single poem file to process. If not provided, processes all .md files in docs/poems/.
    #[arg(short, long, value_name = "FILE_PATH")]
    file: Option<PathBuf>,

    /// Maximum allowed percentage of content reduction. Aborts if reduction exceeds this. Defaults to 1.0.
    #[arg(long, value_name = "PERCENTAGE")]
    max_change_percentage: Option<f64>,

    /// Enable debug output, dumping findings in YAML format.
    #[arg(long)]
    debug: bool,

    /// Perform a dry run, showing changes without writing to disk.
    #[arg(long)]
    dry_run: bool,

    /// Use direct YAML parsing fast path.
    #[arg(long)]
    fast_parse: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PoemReportEntry {
    file_path: String,
    status: String,
    matched_patterns: Option<Vec<String>>,
    error_message: Option<String>,
    extracted_words_count: Option<usize>,
    dry_run_changes_applied: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PoemReport {
    files: Vec<PoemReportEntry>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    let mut regex_config = get_default_regex_config();

    let external_config_path = current_dir.join("regex_config.toml");
    if external_config_path.exists() {
        println!("Loading external regex config from: {external_config_path:?}");
        let external_config = functions::load_regex_config::load_regex_config(&external_config_path)?;

        for external_entry in external_config.regexes {
            if let Some(default_entry) = regex_config.regexes.iter_mut().find(|e| e.name == external_entry.name) {
                *default_entry = external_entry;
            } else {
                regex_config.regexes.push(external_entry);
            }
        }
    }

    use crate::functions::types::PoemFunctionRegistry;
    let function_registry: PoemFunctionRegistry = create_function_registry();

    let mut report_entries: Vec<PoemReportEntry> = Vec::new();

    if cli.fast_parse {
        let file_path = cli.file.ok_or_else(|| anyhow::anyhow!("A file path must be provided for fast parsing."))?;
        match process_single_poem_file_for_report(
            &file_path,
            &regex_config,
            &function_registry,
            cli.debug,
        ) {
            Ok(matched_regexes) => {
                report_entries.push(PoemReportEntry {
                    file_path: file_path.to_string_lossy().into_owned(),
                    status: "Success".to_string(),
                    matched_patterns: Some(matched_regexes),
                    error_message: None,
                    extracted_words_count: None, // Not directly available from this function
                    dry_run_changes_applied: true,
                });
            }
            Err(e) => {
                report_entries.push(PoemReportEntry {
                    file_path: file_path.to_string_lossy().into_owned(),
                    status: "Failed".to_string(),
                    matched_patterns: None,
                    error_message: Some(format!("{}", e)),
                    extracted_words_count: None,
                    dry_run_changes_applied: false,
                });
            }
        }
    } else if let Some(file_path) = cli.file {
        // This branch calls process_poem_file which doesn't return matched regexes directly
        // For now, we'll just report success/failure based on the function's result
        let result = functions::process_poem_file::process_poem_file(
            &file_path,
            cli.max_change_percentage,
            cli.debug,
            cli.dry_run,
            &regex_config,
            &function_registry,
        );
        match result {
            Ok(_) => {
                report_entries.push(PoemReportEntry {
                    file_path: file_path.to_string_lossy().into_owned(),
                    status: "Success".to_string(),
                    matched_patterns: None,
                    error_message: None,
                    extracted_words_count: None,
                    dry_run_changes_applied: cli.dry_run,
                });
            }
            Err(e) => {
                report_entries.push(PoemReportEntry {
                    file_path: file_path.to_string_lossy().into_owned(),
                    status: "Failed".to_string(),
                    matched_patterns: None,
                    error_message: Some(format!("{}", e)),
                    extracted_words_count: None,
                    dry_run_changes_applied: cli.dry_run,
                });
            }
        }
    } else {
        for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                if path.file_name().is_some_and(|name| name == "index.md") {
                    continue;
                }

                match process_single_poem_file_for_report(
                    &path.to_path_buf(),
                    &regex_config,
                    &function_registry,
                    cli.debug,
                ) {
                    Ok(matched_regexes) => {
                        report_entries.push(PoemReportEntry {
                            file_path: path.to_string_lossy().into_owned(),
                            status: "Success".to_string(),
                            matched_patterns: Some(matched_regexes),
                            error_message: None,
                            extracted_words_count: None, // Not directly available from this function
                            dry_run_changes_applied: true,
                        });
                    }
                    Err(e) => {
                        report_entries.push(PoemReportEntry {
                            file_path: path.to_string_lossy().into_owned(),
                            status: "Failed".to_string(),
                            matched_patterns: None,
                            error_message: Some(format!("{}", e)),
                            extracted_words_count: None,
                            dry_run_changes_applied: false,
                        });
                    }
                }
            }
        }
    }

    let report = PoemReport { files: report_entries };
    let report_yaml = serde_yaml::to_string(&report)?;
    let report_path = current_dir.join("poem_processing_report.yaml");
    std::fs::write(&report_path, report_yaml)?;

    println!("Report generated at: {:?}", report_path);

    Ok(())
}
