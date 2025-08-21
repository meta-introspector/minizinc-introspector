use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[test]
fn test_macro_generated_code_compilation() -> Result<()> {
    // 1. Define the input function for the macro.
    let input_fn_code = r#"
fn my_test_function(
    _line: &str,
    _captures: Vec<String>,
    _fixed_fm: &mut HashMap<String, String>,
) -> Result<()> {
    Ok(())
}
"#;

    // 2. Manually expand the macro (simulating the macro's output).
    // This part needs to accurately reflect the current state of poem_macro_impl/src/lib.rs
    let expanded_code = format!(
        r#"{}

#[doc(hidden)]
pub fn __get_fn_my_test_function() -> Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static> {{
    Box::new(|line, captures, fixed_fm| {{
        my_test_function(line, captures, fixed_fm)
    }})
}}

pub static __REGISTER_FN_my_test_function: &'static (String, fn() -> Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>) = &{{
    (stringify!(my_test_function).to_string(), __get_fn_my_test_function)
}};
"#,
        input_fn_code
    );

    // 3. Write the expanded code to a temporary file.
    let temp_dir = tempfile::tempdir()?;
    let file_path = temp_dir.path().join("generated_code.rs");
    fs::write(&file_path, expanded_code)?;

    // 4. Attempt to compile the temporary file using rustc.
    let output = Command::new("rustc")
        .arg(&file_path)
        .arg("--crate-type=lib")
        .output()?;

    // 5. Assert that the compilation was successful.
    if !output.status.success() {
        eprintln!("Compilation failed:");
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success());

    Ok(())
}
