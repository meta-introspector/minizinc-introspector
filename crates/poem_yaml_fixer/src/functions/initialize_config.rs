use anyhow::Result;
use std::path::PathBuf;
use crate::create_function_registry;
use crate::functions::types::PoemFunctionRegistry;
use poem_traits::RegexConfig; // Keep RegexConfig for return type
use crate::functions::register_new_callbacks::register_new_callbacks; // New import

pub fn initialize_config(_cli_manual_parse: bool, _current_dir: &PathBuf) -> Result<(RegexConfig, PoemFunctionRegistry)> {
    // For now, we will only use the function registry populated by macros.
    // The RegexConfig will be an empty default.
    let regex_config = RegexConfig { regexes: Vec::new() };
    let mut function_registry = create_function_registry();

    // Register new callbacks from #[poem_function] macro
    register_new_callbacks(&mut function_registry);

    Ok((regex_config, function_registry))
}
