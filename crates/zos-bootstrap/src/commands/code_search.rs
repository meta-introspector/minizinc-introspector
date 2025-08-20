use clap::Args;
use std::path::PathBuf;
use walkdir::WalkDir;
use regex::Regex;
use crate::utils::error::Result;

#[derive(Args, Clone)]
pub struct CodeSearchArgs {
    pub pattern: String,
    /// Optional: A glob pattern to filter which files are searched (e.g., '*.rs', '*.{cpp,h}').
    #[arg(long)]
    pub include: Option<String>,
    /// Optional: The directory to search within. Defaults to the current working directory.
    #[arg(long)]
    pub path: Option<PathBuf>,
    /// Optional: Output the search results to a MiniZinc DZN file.
    #[arg(long)]
    pub output_dzn: Option<PathBuf>,
}

pub fn handle_code_search_command(args: CodeSearchArgs) -> Result<()> {
    let search_path = match args.path {
        Some(p) => p,
        None => std::env::current_dir()?,
    };

    let regex = Regex::new(&args.pattern)?;

    println!("\n--- Starting Code Search ---");
    println!("Searching for pattern: '{}' in directory: '{}'\n", args.pattern, search_path.display());

    let files_to_search: Vec<PathBuf>;

    if let Some(include_pattern) = args.include {
        // Construct the full glob pattern relative to the search_path
        let full_glob_pattern = search_path.join(&include_pattern).to_string_lossy().to_string();
        
        files_to_search = glob::glob(&full_glob_pattern)?
            .filter_map(|entry| entry.ok())
            .collect();
    } else {
        // If no include pattern, walk the directory and collect all files
        files_to_search = WalkDir::new(&search_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect();
    }

    let mut dzn_output = String::new();
    dzn_output.push_str("search_results = [\n");

    for file_path in files_to_search {
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            for (line_num, line) in content.lines().enumerate() {
                if regex.is_match(line) {
                    // Append to DZN string
                    dzn_output.push_str(&format!(
                        "  (file_path: '{}', line_number: {}, line_content: '{}'),
",
                        file_path.display(),
                        line_num + 1,
                        // Escape double quotes in line_content for DZN string literal
                        line.replace("\"", "\\\"")
                    ));
                }
            }
        }
    }

    // Remove trailing comma and add closing bracket
    if dzn_output.ends_with(",\n") {
        dzn_output.pop(); // Remove newline
        dzn_output.pop(); // Remove comma
    }
    dzn_output.push_str("\n];\n");
    dzn_output.push_str(&format!("pattern_searched = \"{}\";\n", args.pattern.replace("\"", "\\\"")));


    if let Some(output_path) = args.output_dzn {
        std::fs::write(&output_path, dzn_output)?;
        println!("\n--- Code Search Results written to: {} ---", output_path.display());
    } else {
        println!("\n--- Code Search Results ---");
        println!("{}", dzn_output);
    }

    println!("\n--- Code Search Completed ---");
    Ok(())
}
