use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use walkdir::WalkDir;

// Define structs to match the JSON output of llvm-cov export
#[derive(Debug, Deserialize)]
struct LlvmCovFile {
    filename: String,
    segments: Vec<LlvmCovSegment>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
struct LlvmCovSegment {
    line: u32,
    column: u32,
    count: u64,
    has_code: bool,
    is_region_entry: bool,
    // Add other fields as needed
}

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Find .profdata file and C++ binary
    let profdata_path = PathBuf::from("./merged.profdata"); // Assuming merged.profdata is in root
    let binary_path = PathBuf::from("./build_coverage/libminizinc_c_wrapper.so"); // Adjust path as needed

    if !profdata_path.exists() {
        eprintln!("Error: merged.profdata not found at {:?}", profdata_path);
        return Ok(())
    }
    if !binary_path.exists() {
        eprintln!("Error: libminizinc_c_wrapper.so not found at {:?}", binary_path);
        return Ok(())
    }

    // 2. Execute llvm-cov export
    let output = Command::new("llvm-cov")
        .arg("export")
        .arg("-instr-profile")
        .arg(&profdata_path)
        .arg(&binary_path)
        .arg("-format=json")
        .output()?;

    if !output.status.success() {
        eprintln!("llvm-cov export failed: {:?}", output);
        return Err("llvm-cov export failed".into());
    }

    let json_output = String::from_utf8(output.stdout)?;
    let coverage_data: Value = serde_json::from_str(&json_output)?;

    // 3. Parse JSON and extract relevant data
    // This part will be complex, as the JSON structure can be nested.
    // We need to iterate through files and their segments.
    let files_data = coverage_data["data"][0]["files"].as_array().ok_or("Invalid JSON structure")?;

    let mut cpp_file_names: Vec<String> = Vec::new();
    let mut cpp_file_covered: Vec<bool> = Vec::new();
    // More complex data structures needed for per-line/per-test coverage

    for file_obj in files_data {
        let filename = file_obj["filename"].as_str().ok_or("Missing filename")?.to_string();
        // Filter for relevant C++ files (e.g., in lib/ directory)
        if filename.contains("lib/") && filename.ends_with(".cpp") {
            cpp_file_names.push(filename.clone());
            // Determine if file is covered (e.g., if any segment has count > 0)
            let segments = file_obj["segments"].as_array().ok_or("Missing segments")?;
            let is_covered = segments.iter().any(|s| s["count"].as_u64().unwrap_or(0) > 0);
            cpp_file_covered.push(is_covered);
        }
    }

    // 4. Map to MiniZinc data and generate .dzn
    // This will require a more sophisticated mapping for test_covers_file
    // which implies knowing which .profraw came from which test.
    // For now, we'll stick to the simplified file-level coverage.

    let dzn_content = format!(
        "num_cpp_files = {};\n",
        cpp_file_names.len()
    )
    .to_string() + &format!(
        "cpp_file_names = {:?};\n",
        cpp_file_names
    )
    .to_string() + &format!(
        "cpp_file_covered = {:?};\n",
        cpp_file_covered
    )
    .to_string() + &format!(
        "num_rust_tests = {};\n", // This needs to be dynamically determined from test runs
        3 // Placeholder for now
    )
    .to_string() + &format!(
        "rust_test_names = {:?};\n", // Placeholder for now
        vec!["test_parser_basic", "test_model_creation", "test_solver_init"]
    )
    .to_string() + &format!(
        "test_covers_file = [| true, false, false, false, false\n                   | false, true, false, false, false\n                   | false, false, false, false, true |];" // This is the hardest part to automate per-test
    );

    std::fs::write("./extracted_coverage.dzn", dzn_content)?;

    println!("Successfully extracted coverage data to extracted_coverage.dzn");

    Ok(())
}
