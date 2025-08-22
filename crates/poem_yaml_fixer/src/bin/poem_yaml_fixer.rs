use std::path::{Path, PathBuf};
use clap::Parser;
use walkdir::WalkDir;
use poem_yaml_fixer::functions::types::{FixedFrontMatter, PoemFunctionRegistry};
use regex::Regex;
use poem_yaml_fixer::functions::utils::option_vec_helpers::{is_option_vec_empty, extend_option_vec};
use poem_yaml_fixer::process_file;

poem_macros::poem_header!();

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

    /// Generate a report of processed files and matched patterns.
    #[arg(long)]
    report: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    let mut regex_config = get_default_regex_config();

    let external_config_path = current_dir.join("regex_config.toml");
    if external_config_path.exists() {
        println!("Loading external regex config from: {:?}", external_config_path);
        let external_config = poem_yaml_fixer::functions::load_regex_config::load_regex_config(&external_config_path)?;

        for external_entry in external_config.regexes {
            if let Some(default_entry) = regex_config.regexes.iter_mut().find(|e| e.name == external_entry.name) {
                *default_entry = external_entry;
            } else {
                regex_config.regexes.push(external_entry);
            }
        }
    }

    let function_registry: PoemFunctionRegistry = create_function_registry();

    let mut report_entries: Vec<poem_yaml_fixer::functions::report_generator::PoemReportEntry> = Vec::new();

    if let Some(file_path) = cli.file {
        process_file(&file_path, &regex_config, &function_registry, &mut report_entries, cli.debug, cli.dry_run)?;
    } else {
        for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                if path.file_name().is_some_and(|name| name.to_str().unwrap_or("").ends_with(".archeology.md")) {
                    continue;
                }
                process_file(path, &regex_config, &function_registry, &mut report_entries, cli.debug, cli.dry_run)?;
            }
        }
    }

    if cli.report {
        poem_yaml_fixer::functions::report_generator::generate_and_save_report(report_entries, &current_dir)?;
    }

    Ok(())
}
