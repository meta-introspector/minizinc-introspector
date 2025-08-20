use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_recognizer_index(generated_files: Vec<String>, out_dir: &PathBuf, project_root: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dest_path = out_dir.join("generated_recognizer.rs");
    let template_path = PathBuf::from(project_root).join("crates/zos-fast-query/recognizer_template.rs");
    let mut template_content = fs::read_to_string(&template_path)?;

    let mut files_list_str = String::new();
    for file_name in generated_files {
        files_list_str.push_str(&format!("    \"{{}}\"\n", file_name));
    }

    template_content = template_content.replace("// GENERATED_TERM_FILES_PLACEHOLDER", &files_list_str);

    fs::write(&dest_path, template_content)?;

    println!("cargo:warning=Generated recognizer index file at: {:?}", dest_path);
    Ok(())
}
