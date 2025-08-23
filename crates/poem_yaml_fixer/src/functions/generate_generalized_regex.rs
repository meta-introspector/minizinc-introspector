use anyhow::Result;
use std::path::PathBuf;
use std::io::Write;
use grex::RegExpBuilder;

pub fn generate_and_save_generalized_regex(
    unmatched_lines: &[String],
    current_dir: &PathBuf,
) -> Result<()> {
    let regex_templates_dir = current_dir.join("crates/poem_yaml_fixer/src/regex_templates");
    std::fs::create_dir_all(&regex_templates_dir)?;

    println!("DEBUG: Starting grex regex generation.");
    let all_unmatched_lines_str = unmatched_lines.join("\n");
    println!("DEBUG: Joined {} unmatched lines into a single string.", unmatched_lines.len());
    let generated_regex_pattern = RegExpBuilder::from(&[&all_unmatched_lines_str]).build();
    println!("DEBUG: Finished grex regex generation.");

    let module_name = "generalized_unmatched_regex".to_string();
    let function_name = "generalized_unmatched_regex".to_string();

    let file_path = regex_templates_dir.join(format!("{}.rs", module_name));
    
    let file_content = format!(
        r###"// {}
use regex::Regex;
use std::collections::HashMap;

pub fn {}() -> (Regex, HashMap<String, String>) {{
    let regex = Regex::new(r#"{}"#).unwrap();
    let mut captures = HashMap::new();
    // TODO: Populate captures based on the regex groups if any
    (regex, captures)
}}
"###,
        file_path.display(),
        function_name,
        generated_regex_pattern
    );

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(file_content.as_bytes())?;
    println!("Created new generalized regex template file: {:?}\n", file_path);

    // Update the regex_templates/mod.rs file
    let mod_rs_path = regex_templates_dir.join("mod.rs");
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mod_rs_path)?;
    
    writeln!(mod_rs_file, "pub mod {};\n", module_name)?;
    println!("Updated {:?} with new module declaration for generalized regex.\n", mod_rs_path);

    Ok(())
}
