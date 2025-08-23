use anyhow::Result;
use std::path::PathBuf;
use crate::functions::load_regex_config::{get_default_regex_config, load_regex_config};
use crate::create_function_registry;
use crate::functions::types::PoemFunctionRegistry;
use crate::RegexConfig;
use crate::functions::register_new_callbacks::register_new_callbacks; // New import

pub fn initialize_config(cli_manual_parse: bool, current_dir: &PathBuf) -> Result<(RegexConfig, PoemFunctionRegistry)> {
    let (regex_config, mut function_registry) = if cli_manual_parse { // Make function_registry mutable
        (get_default_regex_config(), create_function_registry())
    } else {
        let mut regex_config = get_default_regex_config();

        let external_config_path = current_dir.join("regex_config.toml");
        if external_config_path.exists() {
            println!("Loading external regex config from: {:?}", external_config_path);
            let external_config = load_regex_config(&external_config_path)?;

            for external_entry in external_config.regexes {
                if let Some(default_entry) = regex_config.regexes.iter_mut().find(|e| e.name == external_entry.name) {
                    *default_entry = external_entry;
                } else {
                    regex_config.regexes.push(external_entry);
                }
            }
        }
        (regex_config, create_function_registry())
    };

    // Register new callbacks
    register_new_callbacks(&mut function_registry); // Call the new function

    Ok((regex_config, function_registry))
}
