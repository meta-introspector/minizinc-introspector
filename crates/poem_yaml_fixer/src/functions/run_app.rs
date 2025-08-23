use anyhow::Result;
use std::path::PathBuf;
use crate::functions::types::PoemFunctionRegistry;
use crate::RegexConfig;
//use crate::functions::report_processing::process_poems_for_report;
use crate::functions::process_files::process_files;

pub fn run_app(
    cli_report: bool,
    cli_manual_parse: bool,
    cli_file: &Option<PathBuf>,
    cli_debug: bool,
    cli_dry_run: bool,
    cli_generate_grex_regex: bool,
    poems_dir: &PathBuf,
    current_dir: &PathBuf,
    _regex_config: Option<&RegexConfig>,
    function_registry: &PoemFunctionRegistry,
    log_dir: &PathBuf, // Change from &Option<PathBuf> to &PathBuf
) -> Result<()> {
    process_files(
        cli_file,
        cli_manual_parse,
        cli_debug,
        cli_dry_run,
        cli_report,
        cli_generate_grex_regex,
        poems_dir,
        current_dir,
        _regex_config,
        function_registry,
        log_dir, // Pass the new argument
    )?;
    Ok(())
}
