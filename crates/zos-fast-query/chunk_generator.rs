use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;

const MAX_TERMS_PER_CHUNK: usize = 50_000; // Example limit, adjust as needed
pub fn sanitize_filename_char(c: char) -> String {
    if c.is_ascii_alphanumeric() {
        c.to_string()
    } else {
        format!("U{:04X}", c as u32) // Format as UXXXX (hex Unicode codepoint)
    }
}

pub fn sanitize_filename(s: &str) -> String {
    s.chars().map(|c| sanitize_filename_char(c)).collect::<Vec<String>>().join("")
}
pub fn generate_term_chunks(filtered_terms: Vec<String>, out_dir: &PathBuf) -> Result<(Vec<String>, u64), Box<dyn std::error::Error>> {
    let mut terms_by_first_char: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for term in filtered_terms {
        if let Some(first_char) = term.chars().next() {
            let sanitized_char = sanitize_filename(&first_char.to_string());
            terms_by_first_char.entry(sanitized_char).or_insert_with(Vec::new).push(term);
        }
    }

    let mut generated_files = Vec::new();
    let mut total_generated_size: u64 = 0;

    for (sanitized_char, terms) in terms_by_first_char {
        if terms.len() > MAX_TERMS_PER_CHUNK {
            // Split into smaller chunks
            println!("cargo:warning=  Splitting terms for '{}' ({} terms) into chunks of {} terms.", sanitized_char, terms.len(), MAX_TERMS_PER_CHUNK);
            for (i, chunk) in terms.chunks(MAX_TERMS_PER_CHUNK).enumerate() {
                let chunk_file_name = format!("terms_{}_{}.json", sanitized_char, i);
                let chunk_file_path = out_dir.join(&chunk_file_name);
                let json_content = serde_json::to_string(&chunk)?;
                fs::write(&chunk_file_path, &json_content)?;
                let file_size = json_content.len() as u64;
                total_generated_size += file_size;
                println!("cargo:warning=    Generated chunk: {} ({} terms, {} bytes)", chunk_file_name, chunk.len(), file_size);
                generated_files.push(chunk_file_name);
            }
        } else {
            // Write directly to a single file
            let file_name = format!("terms_{}.json", sanitized_char);
            let file_path = out_dir.join(&file_name);
            let json_content = serde_json::to_string(&terms)?;
            fs::write(&file_path, &json_content)?;
            let file_size = json_content.len() as u64;
            total_generated_size += file_size;
            println!("cargo:warning=  Generated file: {} ({} terms, {} bytes)", file_name, terms.len(), file_size);
            generated_files.push(file_name);
        }
    }

    println!("cargo:warning=Total generated term files: {}", generated_files.len());
    println!("cargo:warning=Total size of generated term files: {} bytes", total_generated_size);

    Ok((generated_files, total_generated_size))
}
