use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;
use anyhow::anyhow; // Add this line
 // Need fs for checking file existence

poem_macros::poem_header!(); // Call the header macro once

mod functions; // Declare the functions module once

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

    /// Perform a dry run, showing changes without writing to disk.
    #[arg(long)]
    dry_run: bool,

    /// Use direct YAML parsing fast path.
    #[arg(long)]
    fast_parse: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    let mut regex_config = get_default_regex_config(); // Get default config from macro

    // Check for an external regex_config.toml in the current directory
    let external_config_path = current_dir.join("regex_config.toml");
    if external_config_path.exists() {
        println!("Loading external regex config from: {external_config_path:?}");
        let external_config = functions::load_regex_config::load_regex_config(&external_config_path)?;

        // Merge external config into default config (external overrides defaults by name)
        for external_entry in external_config.regexes {
            if let Some(default_entry) = regex_config.regexes.iter_mut().find(|e| e.name == external_entry.name) {
                *default_entry = external_entry; // Override existing entry
            } else {
                regex_config.regexes.push(external_entry); // Add new entry
            }
        }
    }

    use poem_traits::FunctionRegistry; // Import FunctionRegistry
    let function_registry: FunctionRegistry = create_function_registry(); // Use FunctionRegistry

    if cli.fast_parse {
        let file_path = cli.file.ok_or_else(|| anyhow::anyhow!("A file path must be provided for fast parsing."))?;
        println!("Performing fast parse for: {file_path:?}\n");
        let (fixed_fm, poem_body) = functions::parse_poem_file_direct::parse_poem_file_direct(&file_path)?;
        println!("Parsed Front Matter: {fixed_fm:#?}\nExtracted Poem Body: {poem_body}");
    } else if let Some(file_path) = cli.file {
        functions::process_poem_file::process_poem_file(
            &file_path,
            cli.max_change_percentage,
            cli.debug,
            cli.dry_run, // Pass dry_run
            &regex_config,
            &function_registry,
        )?;
    } else {
        for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                if path.file_name().is_some_and(|name| name == "index.md") {
                    continue;
                }

                println!("Processing: {path:?}
");
                let path_buf = path.to_path_buf();
                functions::process_poem_file::process_poem_file(
                    &path_buf,
                    cli.max_change_percentage,
                    cli.debug,
                    cli.dry_run, // Pass dry_run
                    &regex_config,
                    &function_registry,
                )?;
            }
        }
    }

    Ok(())
}