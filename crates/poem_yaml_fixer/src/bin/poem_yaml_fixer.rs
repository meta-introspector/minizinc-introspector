use std::path::{PathBuf};
use clap::Parser;
//use walkdir::WalkDir;
//use poem_yaml_fixer::functions::types::FixedFrontMatter; // Added for manual_parse_poem_file
//use poem_yaml_fixer::manual_parser::manual_parse_poem_file; // Import the manual parser
//use poem_yaml_fixer::functions::callbacks::handle_title_regex::handle_title_regex;
//use poem_yaml_fixer::functions::types::{FixedFrontMatter, PoemFunctionRegistry};
//use regex::Regex;
//use poem_yaml_fixer::functions::utils::option_vec_helpers::{is_option_vec_empty, extend_option_vec};
//use poem_yaml_fixer::functions::process_single_poem_file_for_report::process_single_poem_file_for_report;

//use poem_yaml_fixer::functions::report_processing::process_poems_for_report;
//use poem_yaml_fixer::functions::load_regex_config::get_default_regex_config;
//use poem_yaml_fixer::create_function_registry;
//use poem_yaml_fixer::process_file;

//use poem_yaml_fixer::functions::report_printer::print_detailed_regex_report;
use poem_yaml_fixer::functions::initialize_config::initialize_config;
//use poem_yaml_fixer::functions::process_files::process_files;
use poem_yaml_fixer::functions::run_app::run_app;

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
    #[arg(long, default_value_t = true)]
    dry_run: bool,

    /// Use direct YAML parsing fast path.
    #[arg(long)]
    fast_parse: bool,

    /// Generate a report of processed files and matched patterns.
    #[arg(long)]
    report: bool,

    /// Use the manual parser for testing purposes.
    #[arg(long)]
    manual_parse: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    let (regex_config, function_registry) = initialize_config(cli.manual_parse, &current_dir)?;

    run_app(
        cli.report,
        cli.manual_parse,
        &cli.file,
        cli.debug,
        cli.dry_run,
        &poems_dir,
        &current_dir,
        &regex_config,
        &function_registry,
    )?;

    Ok(())
}
