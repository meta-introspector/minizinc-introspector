use std::path::PathBuf;
use clap::Parser;

poem_macros::poem_header!(); // Call the header macro

mod functions; // Declare the functions module



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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    let regex_config = functions::load_regex_config::load_regex_config()?; // Load regex patterns
    let function_registry = create_function_registry(); // Create the function registry

    if let Some(file_path) = cli.file {
        functions::process_single_file::process_single_file(
            &file_path,
            cli.max_change_percentage,
            cli.debug,
            &regex_config,
            &function_registry,
        )?;
    } else {
        functions::process_all_files::process_all_files(
            &poems_dir,
            cli.max_change_percentage,
            cli.debug,
            &regex_config,
            &function_registry,
        )?;
    }

    Ok(())
}
