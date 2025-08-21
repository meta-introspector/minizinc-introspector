use std::{fs, path::PathBuf,
	  //collections::HashMap
};
use anyhow::{Result
	     // ,anyhow
};
use walkdir::WalkDir;
use clap::Parser;

mod functions; // Declare the functions module

// Import all functions from the functions module
use crate::functions::process_poem_file::process_poem_file;
use crate::functions::create_function_registry::create_function_registry;

// Import common types from the types module
use crate::functions::types::{
    //FixedFrontMatter, Meme,
    RegexConfig,
    //RegexEntry, WordIndex, CallbackFn
};

// Add Cli struct
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    // Load regex patterns from TOML once
    let regex_config_str = fs::read_to_string("crates/poem_yaml_fixer/src/regex_patterns.toml")?;
    let regex_config: RegexConfig = toml::from_str(&regex_config_str)?;

    // Create the function registry once
    let function_registry = create_function_registry();

    if let Some(file_path) = cli.file {
        println!("Processing single file: {:?}", file_path);
        match process_poem_file(
            &file_path,
            cli.max_change_percentage,
            cli.debug,
            &regex_config,
            &function_registry,
        ) {
            Ok(_) => println!("Successfully fixed: {:?}\n", file_path),
            Err(e) => eprintln!("Error fixing {:?}: {}\n", file_path, e),
        }
    } else {
        for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if path.file_name().map_or(false, |name| name == "index.md") {
                    continue;
                }

                println!("Processing: {:?}", path);
                let path_buf = path.to_path_buf();
                match process_poem_file(
                    &path_buf,
                    cli.max_change_percentage,
                    cli.debug,
                    &regex_config,
                    &function_registry,
                ) {
                    Ok(_) => println!("Successfully fixed: {:?}\n", path_buf),
                    Err(e) => eprintln!("Error fixing {:?}: {}\n", path_buf, e),
                }
            }
        }
    }

    Ok(())
}
