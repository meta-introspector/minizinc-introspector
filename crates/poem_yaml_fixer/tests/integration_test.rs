use std::process::Command;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_quality_procedures_sonnet_fix() -> anyhow::Result<()> {
    // Define the path to the poem file
    let poem_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("docs")
        .join("poems")
        .join("quality_procedures_sonnet.md");

    // Ensure the poem file exists
    assert!(poem_path.exists(), "Poem file does not exist: {poem_path:?}");

    // Define a temporary output directory for the test
    let temp_dir = tempfile::tempdir()?;
    let temp_poem_path = temp_dir.path().join("quality_procedures_sonnet.md");

    // Create the temporary directory
    fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");

    // Copy the original poem to the temporary directory for testing
    fs::copy(&poem_path, &temp_poem_path).expect("Failed to copy poem to temporary directory");

    // Run the poem_yaml_fixer in dry-run mode
    let output = Command::new(env!("CARGO_BIN_EXE_poem_yaml_fixer"))
        .arg("--file")
        .arg(&temp_poem_path)
        .arg("--max-change-percentage").arg("90.0")
        .arg("--dry-run")
        .output()
        .expect("Failed to execute poem_yaml_fixer");

    // Assert that the command ran successfully
    assert!(output.status.success(), "Command failed with: {output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Stdout: {stdout}");
    println!("Stderr: {stderr}");

    // --- Assertions for the dry-run output ---
    // In dry-run mode, we expect to see "Would apply changes to" if changes are needed,
    // or "No changes needed for" if the file is already clean.
    // Initially, we expect changes because the YAML is "broken" and needs fixing.

    assert!(stdout.contains("Dry run: Would apply changes to:"), "Expected dry-run to indicate changes.");

    // --- Assertions for the fixed YAML content (after running without dry-run) ---
    // To verify the actual fix, we need to run it *without* dry-run and then read the file.
    // This part will be commented out for now, as the user wants to "grind" on the dry-run first.
    /*
    let output_apply = Command::new(env!("CARGO_BIN_EXE_poem_yaml_fixer"))
        .arg("--file")
        .arg(&temp_poem_path)
        .output()
        .expect("Failed to execute poem_yaml_fixer for applying changes");

    assert!(output_apply.status.success(), "Apply command failed with: {:?}", output_apply);

    let fixed_content = fs::read_to_string(&temp_poem_path).expect("Failed to read fixed poem file");

    // TODO: Add assertions for the expected fixed YAML content
    // This will involve parsing the fixed_content and asserting its structure and values.
    // For example:
    // let fixed_fm: FixedFrontMatter = serde_yaml::from_str(&fixed_content).expect("Failed to parse fixed YAML");
    // assert_eq!(fixed_fm.title, Some("The Quality's Steadfast Code".to_string()));
    // assert!(fixed_fm.memes.len() > 0);
    */

    // Clean up temporary directory
    fs::remove_dir_all(&temp_dir).expect("Failed to remove temporary directory");
    Ok(())
}
