
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

// Define the prime categories and their associated keywords/senses
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum PrimeCategory {
    Two,   // Duality/Foundation
    Three, // Structure/Process/Phases
    Five,  // Workflow/SOPs/OODA
    Seven, // Narrative/Poetry/Mentors
    Eleven, // Change/Evolution/CRQs
    Thirteen, // Verification/Testing/QA
    Seventeen, // Troubleshooting/Issues/Debugging
    Nineteen, // Interface/CLI/Interaction
    Uncategorized,
}

impl PrimeCategory {
    fn to_path_segment(&self) -> &str {
        match self {
            PrimeCategory::Two => "2",
            PrimeCategory::Three => "3",
            PrimeCategory::Five => "5",
            PrimeCategory::Seven => "7",
            PrimeCategory::Eleven => "11",
            PrimeCategory::Thirteen => "13",
            PrimeCategory::Seventeen => "17",
            PrimeCategory::Nineteen => "19",
            PrimeCategory::Uncategorized => "0",
        }
    }

    fn to_sense_map(&self) -> HashMap<&'static str, Vec<&'static str>> {
        let mut map = HashMap::new();
        match self {
            PrimeCategory::Two => {
                map.insert("Core_Concepts", vec!["core", "fundamental", "base", "foundation", "concept", "abstraction", "ontology", "gemini"]);
                map.insert("Binary_Distinctions", vec!["binary", "duality", "internal", "external"]);
            }
            PrimeCategory::Three => {
                map.insert("Architecture_Design", vec!["architecture", "design", "pattern", "model"]);
                map.insert("Build_Process", vec!["build", "compilation", "deployment"]);
                map.insert("Workflow_Phases", vec!["workflow", "phase", "stage"]);
            }
            PrimeCategory::Five => {
                map.insert("Standard_Operating_Procedures", vec!["sop", "procedure", "process"]);
                map.insert("Iterative_Processes", vec!["iterative", "loop", "feedback"]);
                map.insert("OODA_Loop_Context", vec!["ooda"]);
            }
            PrimeCategory::Seven => {
                map.insert("Narratives", vec!["narrative", "story", "journey"]);
                map.insert("Poetry", vec!["poem", "haiku", "sonnet", "limerick"]);
                map.insert("Mentor_Influence", vec!["mentor", "llvm", "linux", "minizinc", "lean4", "rust", "bert", "tclifford", "git", "wikidata", "archive.org", "openstreetmap", "gnu"]);
            }
            PrimeCategory::Eleven => {
                map.insert("Change_Requests", vec!["crq", "change request", "feature", "proposal", "enhancement"]);
                map.insert("Evolution_Proposals", vec!["evolution", "shift", "new feature"]);
            }
            PrimeCategory::Thirteen => {
                map.insert("Testing_Procedures", vec!["test", "qa", "quality", "testing"]);
                map.insert("Verification_Strategies", vec!["verify", "verification", "proof", "validate"]);
            }
            PrimeCategory::Seventeen => {
                map.insert("Troubleshooting_Guides", vec!["troubleshoot", "debug", "fix", "resolve"]);
                map.insert("Bug_Reports", vec!["bug", "issue", "error", "report"]);
            }
            PrimeCategory::Nineteen => {
                map.insert("CLI_Documentation", vec!["cli", "command line", "argument", "usage"]);
                map.insert("External_Interfaces", vec!["interface", "api", "external"]);
            }
            PrimeCategory::Uncategorized => {
                map.insert("Uncategorized", vec![]);
            }
        }
        map
    }
}

fn categorize_file(file_path: &Path) -> (PrimeCategory, Option<String>) {
    let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let file_content = fs::read_to_string(file_path).unwrap_or_default().to_lowercase();

    // Prioritize existing categorized structure
    if let Some(parent) = file_path.parent() {
        if let Some(grandparent) = parent.parent() {
            if grandparent.ends_with("docs/categorized") {
                if let Some(category_num_str) = parent.file_name().and_then(|s| s.to_str()) {
                    if let Ok(category_num) = category_num_str.parse::<u32>() {
                        match category_num {
                            1 => return (PrimeCategory::Two, Some("General_Documentation".to_string())),
                            2 => return (PrimeCategory::Two, Some("Core_Concepts".to_string())),
                            3 => return (PrimeCategory::Three, Some("Structure_Build".to_string())),
                            5 => return (PrimeCategory::Five, Some("Organization_Process".to_string())),
                            11 => return (PrimeCategory::Eleven, Some("Modularity_Design".to_string())), // Re-evaluate this mapping
                            17 => return (PrimeCategory::Seventeen, Some("Abstraction_Ontology".to_string())), // Re-evaluate this mapping
                            19 => return (PrimeCategory::Nineteen, Some("Iteration_Evolution".to_string())), // Re-evaluate this mapping
                            _ => {}
                        }
                    }
                }
            }
        }
    }


    // Check for specific file name patterns first
    if file_name.contains("crq") || file_name.contains("change_request") {
        return (PrimeCategory::Eleven, Some("Change_Requests".to_string()));
    }
    if file_name.contains("test") || file_name.contains("qa") {
        return (PrimeCategory::Thirteen, Some("Testing_Procedures".to_string()));
    }
    if file_name.contains("bug") || file_name.contains("issue") || file_name.contains("troubleshoot") {
        return (PrimeCategory::Seventeen, Some("Troubleshooting_Guides".to_string()));
    }
    if file_name.contains("cli") || file_name.contains("argument") || file_name.contains("usage") {
        return (PrimeCategory::Nineteen, Some("CLI_Documentation".to_string()));
    }
    if file_name.contains("poem") || file_name.contains("narrative") || file_name.contains("journey") {
        return (PrimeCategory::Seven, Some("Narratives".to_string()));
    }
    if file_name.contains("sop") || file_name.contains("process") || file_name.contains("workflow") {
        return (PrimeCategory::Five, Some("Standard_Operating_Procedures".to_string()));
    }
    if file_name.contains("architecture") || file_name.contains("design") || file_name.contains("build") {
        return (PrimeCategory::Three, Some("Architecture_Design".to_string()));
    }
    if file_name.contains("gemini") || file_name.contains("core") || file_name.contains("foundation") {
        return (PrimeCategory::Two, Some("Core_Concepts".to_string()));
    }


    // Analyze content for keywords
    for (category, sense_map) in [
        (PrimeCategory::Two, PrimeCategory::Two.to_sense_map()),
        (PrimeCategory::Three, PrimeCategory::Three.to_sense_map()),
        (PrimeCategory::Five, PrimeCategory::Five.to_sense_map()),
        (PrimeCategory::Seven, PrimeCategory::Seven.to_sense_map()),
        (PrimeCategory::Eleven, PrimeCategory::Eleven.to_sense_map()),
        (PrimeCategory::Thirteen, PrimeCategory::Thirteen.to_sense_map()),
        (PrimeCategory::Seventeen, PrimeCategory::Seventeen.to_sense_map()),
        (PrimeCategory::Nineteen, PrimeCategory::Nineteen.to_sense_map()),
    ] {
        for (sense, keywords) in sense_map {
            for keyword in keywords {
                if file_content.contains(keyword) {
                    return (category, Some(sense.to_string()));
                }
            }
        }
    }

    (PrimeCategory::Uncategorized, Some("Uncategorized".to_string()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc");
    let target_base_dir = root_dir.join("docs/prime_categorized");

    // Collect all .md files, excluding vendor/
    let mut md_files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&root_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.ends_with("vendor") {
            continue; // Skip vendor directory
        }
        if path.is_dir() {
            for sub_entry in fs::walkdir::WalkDir::new(&path) {
                let sub_entry = sub_entry?;
                let sub_path = sub_entry.path();
                if sub_path.is_file() && sub_path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if !sub_path.starts_with(root_dir.join("vendor")) {
                        md_files.push(sub_path.to_path_buf());
                    }
                }
            }
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            md_files.push(path);
        }
    }

    println!("Found {} .md files (excluding vendor/):", md_files.len());

    let mut moves: HashMap<PathBuf, (PrimeCategory, Option<String>)> = HashMap::new();

    for file_path in md_files {
        let (category, sense) = categorize_file(&file_path);
        moves.insert(file_path, (category, sense));
    }

    // Print proposed moves
    for (old_path, (category, sense)) in &moves {
        let new_dir = target_base_dir
            .join(category.to_path_segment())
            .join(sense.as_deref().unwrap_or("Uncategorized"));
        let new_path = new_dir.join(old_path.file_name().unwrap());

        println!(
            "Move: {:?} -> {:?} (Category: {:?}, Sense: {:?})",
            old_path, new_path, category, sense
        );

        // Create parent directories if they don't exist
        fs::create_dir_all(&new_dir)?;

        // Perform the move
        // fs::rename(old_path, new_path)?; // Uncomment to actually move files
    }

    // Create README.md files for each prime category and sense
    for (category, sense_map) in [
        (PrimeCategory::Two, PrimeCategory::Two.to_sense_map()),
        (PrimeCategory::Three, PrimeCategory::Three.to_sense_map()),
        (PrimeCategory::Five, PrimeCategory::Five.to_sense_map()),
        (PrimeCategory::Seven, PrimeCategory::Seven.to_sense_map()),
        (PrimeCategory::Eleven, PrimeCategory::Eleven.to_sense_map()),
        (PrimeCategory::Thirteen, PrimeCategory::Thirteen.to_sense_map()),
        (PrimeCategory::Seventeen, PrimeCategory::Seventeen.to_sense_map()),
        (PrimeCategory::Nineteen, PrimeCategory::Nineteen.to_sense_map()),
        (PrimeCategory::Uncategorized, PrimeCategory::Uncategorized.to_sense_map()),
    ] {
        for (sense, _) in sense_map {
            let readme_dir = target_base_dir.join(category.to_path_segment()).join(sense);
            let readme_path = readme_dir.join("README.md");
            let mut readme_content = format!("# Prime Category: {:?}

", category);
            readme_content.push_str(&format!("## Sense: {}

", sense));
            readme_content.push_str("### Files in this category:

");

            for (old_path, (file_category, file_sense)) in &moves {
                if file_category == &category && file_sense.as_deref() == Some(sense) {
                    readme_content.push_str(&format!("- {}
", old_path.display()));
                }
            }
            fs::write(&readme_path, readme_content)?;
        }
    }

    println!("Generated Rust program to categorize and propose moves for .md files. Review the output and uncomment `fs::rename` to perform the moves.");
    println!("To compile and run this program:");
    println!("1. Navigate to the project root: `cd /data/data/com.termux/files/home/storage/github/libminizinc`");
    println!("2. Compile: `cargo build`");
    println!("3. Run: `cargo run`");

    Ok(())
}
