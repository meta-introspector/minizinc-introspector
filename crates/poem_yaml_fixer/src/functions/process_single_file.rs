use anyhow::Result;
use std::path::PathBuf;
use crate::functions::process_poem_file::process_poem_file;
use crate::functions::error_handling::handle_unmatched_regex_error::handle_unmatched_regex_error;
use crate::functions::types::RegexConfig;
use std::collections::HashMap; // For function_registry

pub fn process_single_file(
    file_path: &PathBuf,
    max_change_percentage: Option<f64>,
    debug_mode: bool,
    regex_config: &RegexConfig,
    function_registry: &HashMap<String, Box<dyn Fn(&str, Vec<String>, &mut dyn poem_traits::PoemFrontMatterTrait) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>>,
) -> Result<()> {
    println!("Processing single file: {:?}", file_path);
    match process_poem_file(
        file_path,
        max_change_percentage,
        debug_mode,
        regex_config,
        function_registry,
    ) {
        Ok(_) => println!("Successfully fixed: {:?}\n", file_path),
        Err(e) => {
            if e.to_string().contains("No regex matched line:") {
                handle_unmatched_regex_error(file_path, &e.to_string())?;
            } else {
                eprintln!("Error fixing {:?}: {}
", file_path, e);
            }
        }
    }
    Ok(())
}
