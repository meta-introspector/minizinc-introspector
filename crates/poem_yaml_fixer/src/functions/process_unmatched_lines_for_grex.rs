use anyhow::Result;
use std::path::{Path, PathBuf};
use std::io::Write;
use grex::RegExpBuilder;
use regex::Regex; // For alphanumeric word splitting

pub fn process_unmatched_lines_for_grex(
    unmatched_lines: &[String],
    file_path: &Path,
    current_dir: &Path,
    log_dir: &PathBuf, // Change from &Option<PathBuf> to &PathBuf
) -> Result<()> {
    if unmatched_lines.is_empty() {
        return Ok(())
    }

    let mut sampled_lines: Vec<String> = Vec::new();
    let alphanumeric_word_regex = Regex::new(r"[a-zA-Z0-9]+")?;

    for line in unmatched_lines {
        let mut words_count = 0;
        let mut sampled_line_parts = Vec::new();
        for mat in alphanumeric_word_regex.find_iter(line) {
            if words_count < 3 {
                sampled_line_parts.push(mat.as_str());
                words_count += 1;
            } else {
                break;
            }
        }
        if !sampled_line_parts.is_empty() {
            sampled_lines.push(sampled_line_parts.join(" "));
        }
    }

    if sampled_lines.is_empty() {
        return Ok(())
    }

    let all_sampled_lines_str = sampled_lines.join("\n");
    let generated_regex_pattern = RegExpBuilder::from(&[&all_sampled_lines_str]).build();

    let poem_file_name = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown_poem");
    let output_file_name = format!("grex_regex_{}.rs", poem_file_name);

    let regex_templates_dir = current_dir.join("crates/poem_yaml_fixer/src/regex_templates");
    std::fs::create_dir_all(&regex_templates_dir)?;

    let file_path = regex_templates_dir.join(&output_file_name);
    
    let file_content = format!(
        r###"// Generated regex for unmatched lines in: {}\nuse regex::Regex;\nuse std::collections::HashMap;\n\npub fn {}_regex() -> (Regex, HashMap<String, String>) {{\n    let regex = Regex::new(r#"{}"#).unwrap();\n    let mut captures = HashMap::new();\n    // TODO: Populate captures based on the regex groups if any\n    (regex, captures)\n}}\n"###,
        file_path.display(),
        poem_file_name,
        generated_regex_pattern
    );

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(file_content.as_bytes())?;
    crate::write_to_log_file(log_dir, &file_path, &format!("Generated poem-specific grex regex for {:?} at: {:?}\n", file_path, file_path))?;

    // Update the regex_templates/mod.rs file
    let mod_rs_path = regex_templates_dir.join("mod.rs");
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mod_rs_path)?;
    
    writeln!(mod_rs_file, "pub mod {};", poem_file_name)?;
    crate::write_to_log_file(log_dir, &file_path, &format!("Updated {:?} with new module declaration for poem-specific grex regex.\n", mod_rs_path))?;

    Ok(())
}
