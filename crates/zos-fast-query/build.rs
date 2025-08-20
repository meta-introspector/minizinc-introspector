use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Rerun this build script if the hierarchical_term_index.json changes
    println!("cargo:rerun-if-changed=/data/data/com.termux/files/home/storage/github/hierarchical_term_index.json");

    // Get the path to the zos-fast-query executable
    let cargo_bin_dir = env::var("CARGO_BIN_EXE_zos-fast-query")
        .expect("CARGO_BIN_EXE_zos-fast-query not set");
    let zos_fast_query_exe = Path::new(&cargo_bin_dir);

    // Run the compile-terms-regex command
    let output = Command::new(zos_fast_query_exe)
        .arg("compile-terms-regex")
        .output()
        .expect("Failed to execute compile-terms-regex command");

    if !output.status.success() {
        panic!(
            "compile-terms-regex failed: stdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:warning=compile-terms-regex output: {}", String::from_utf8_lossy(&output.stdout));
}

