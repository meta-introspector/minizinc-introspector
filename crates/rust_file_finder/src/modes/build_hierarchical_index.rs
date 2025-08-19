pub fn run_build_hierarchical_index_mode() -> Result<()> {
    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");
    let hierarchical_term_index_file = search_root.join("hierarchical_term_index.json");

    eprintln!("Discovering Rust projects in: {:?}", search_root);

    // First pass: Discover Cargo.toml files to identify project roots
    let mut discovered_project_roots: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().components().any(|comp| comp.as_os_str() == "target")) // Skip target directories
    {
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |name| name == "Cargo.toml") {
            if let Some(parent) = path.parent() {
                discovered_project_roots.insert(parent.to_path_buf());
            }
        }
    }

    let mut all_rust_files: Vec<FileAnalysis> = Vec::new();

    // Load project summaries
    for project_root in discovered_project_roots {
        let project_summary_file = project_root.join(".file_analysis_summary.json");
        if project_summary_file.exists() {
            if let Ok(cached_data) = fs::read_to_string(&project_summary_file) {
                if let Ok(project_analysis) = serde_json::from_str::<ProjectAnalysis>(&cached_data) {
                    all_rust_files.extend(project_analysis.rust_files.into_iter());
                }
            }
        }
    }

    if all_rust_files.is_empty() {
        eprintln!("No Rust files found in cache. Run full_analysis first.");
        return Ok(());
    }

    let mut hierarchical_term_index: HashMap<String, HashSet<PathBuf>> = HashMap::new();

    for file_analysis in &all_rust_files {
        for (word, _count) in &file_analysis.bag_of_words {
            let mut current_path = Some(file_analysis.path.clone());
            while let Some(path) = current_path {
                hierarchical_term_index.entry(word.clone()).or_insert_with(HashSet::new).insert(path.clone());
                current_path = path.parent().map(|p| p.to_path_buf());
            }
        }
    }

    eprintln!("Saving hierarchical term index to {:?}", hierarchical_term_index_file);
    let serialized_index = serde_json::to_string_pretty(&hierarchical_term_index)?;
    fs::write(&hierarchical_term_index_file, serialized_index)?;

    Ok(())
}
