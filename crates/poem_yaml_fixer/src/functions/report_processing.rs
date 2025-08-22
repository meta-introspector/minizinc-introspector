use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::WalkDir;

use poem_traits::{RegexConfig, FunctionRegistry};
use crate::functions::process_single_poem_file_for_report::process_single_poem_file_for_report;

pub fn process_poems_for_report(
    poems_dir: &Path,
    file_path_cli: &Option<PathBuf>,
    regex_config: &RegexConfig,
    function_registry: &FunctionRegistry,
    debug_mode: bool,
) -> Result<()> {
    if let Some(file_path) = file_path_cli {
        let matched_patterns = process_single_poem_file_for_report(
            file_path,
            regex_config,
            function_registry,
            debug_mode,
        )?;
        println!("Report for {:?}: Matched patterns: {:?}", file_path, matched_patterns);
    } else {
        for entry in WalkDir::new(poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                if path.file_name().is_some_and(|name| name.to_str().unwrap_or("").ends_with(".archeology.md")) {
                    continue;
                }
                let matched_patterns = process_single_poem_file_for_report(
                    &path.to_path_buf(), // Convert &Path to &PathBuf
                    regex_config,
                    function_registry,
                    debug_mode,
                )?;
                println!("Report for {:?}: Matched patterns: {:?}", path, matched_patterns);
            }
        }
    }
    Ok(())
}
