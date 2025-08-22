use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::functions::process_single_file::process_single_file;
use crate::functions::types::RegexConfig;
use std::collections::HashMap; // For function_registry

pub fn process_all_files(
    poems_dir: &PathBuf,
    max_change_percentage: Option<f64>,
    debug_mode: bool,
    regex_config: &RegexConfig,
    function_registry: &HashMap<String, Box<dyn Fn(&str, Vec<String>, &mut dyn poem_traits::PoemFrontMatterTrait) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>>,
) -> Result<()> {
    for entry in WalkDir::new(poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Processing: {:?}", path);
            let path_buf = path.to_path_buf();
            process_single_file(
                &path_buf,
                max_change_percentage,
                debug_mode,
                regex_config,
                function_registry,
            )?;
        }
    }
    Ok(())
}
