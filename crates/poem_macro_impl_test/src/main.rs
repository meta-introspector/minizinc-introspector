use anyhow::Result;
use syn::{parse_quote, ItemFn};
use poem_macro_impl::poem_function_impl;
//use poem_macro_impl::poem_function_impl;
use std::fs;
use std::process::Command;
use proc_macro2::TokenStream;

fn main() -> Result<()> {
    // Define a dummy function as if it were input to the macro
    let input_fn: ItemFn = parse_quote! {
        fn my_dummy_function(
            _line: &str,
            captures: Vec<String>,
            fixed_fm: &mut std::collections::HashMap<String, String>,
        ) -> anyhow::Result<(), anyhow::Error> {
            // This is a dummy implementation
            println!("Dummy function executed: line={}, captures={:?}", _line, captures);
            fixed_fm.insert("dummy_key".to_string(), "dummy_value".to_string());
            Ok(())
        }
    };

    let attr_tokens: TokenStream = TokenStream::new();

    // Call the macro implementation function directly
    let expanded_tokens = poem_function_impl(attr_tokens, input_fn);

    // Convert TokenStream to String for writing to file
    let expanded_code_string = expanded_tokens.to_string();

    // Create a temporary directory for the new cargo project
    let temp_project_dir = tempfile::tempdir()?;

    // Create Cargo.toml for the temporary project
    let cargo_toml_content = r#"
[package]
name = "temp_generated_code"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
"#;
    fs::write(temp_project_dir.path().join("Cargo.toml"), cargo_toml_content)?;

    // Create src directory
    let temp_src_dir = temp_project_dir.path().join("src");
    fs::create_dir(&temp_src_dir)?;

    // Write the generated code to src/lib.rs
    fs::write(temp_src_dir.join("lib.rs"), expanded_code_string)?;

    // Run cargo build in the temporary project directory
    let output = Command::new("cargo")
        .arg("build")
        .current_dir(&temp_project_dir)
        .output()?;

    // Assert that the compilation was successful
    if !output.status.success() {
        eprintln!("Compilation failed for generated code:");
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        // Exit with a non-zero code to indicate test failure
        std::process::exit(1);
    }

    println!("Generated code compiled successfully!");

    Ok(())
}
