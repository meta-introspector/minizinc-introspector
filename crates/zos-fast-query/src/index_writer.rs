use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_recognizer_index(generated_files: Vec<String>, out_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let template_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/crates/zos-fast-query/src/recognizer_template.rs");
    let mut template_content = fs::read_to_string(&template_path)?;

    let mut files_list_str = String::new();
    for file_name in generated_files {
        files_list_str.push_str(&format!("    \"{}\"", file_name));
    }

    template_content = template_content.replace("// GENERATED_TERM_FILES_PLACEHOLDER", &files_list_str);

    fs::write(&dest_path, template_content)?;

    println!("cargo:warning=Generated recognizer index file at: {:?}", dest_path);
    Ok(())
}
