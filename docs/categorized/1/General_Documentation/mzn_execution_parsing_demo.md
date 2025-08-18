# Executing MiniZinc and Parsing Output

## Objective
To detail the process of executing the generated MiniZinc model (`.mzn`) and data file (`.dzn`), and subsequently parsing the solver's output to retrieve the `suggested_numerical_vector`.

## Process Overview
As outlined in `crates/zos-bootstrap/src/commands/ast_to_minizinc.rs`, after the `.mzn` and `.dzn` files are generated, the `minizinc` executable is invoked. Its standard output is then captured and parsed to retrieve the `suggested_numerical_vector`.

## Execution Details

The relevant code snippet for execution is:

```rust
    println!("\nPhase 5: Executing MiniZinc...");
    let libminizinc_build_dir = paths::get_build_dir()?;
    let minizinc_exe = libminizinc_build_dir.join("minizinc");

    let args_mzn = vec![
        model_file_path.to_string_lossy().to_string(),
        data_file_path.to_string_lossy().to_string(),
    ];

    let args_str: Vec<&str> = args_mzn.iter().map(|s| s.as_ref()).collect();

    let output = subprocess::run_command(&minizinc_exe.to_string_lossy(), &args_str)?;

    println!("MiniZinc Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("MiniZinc Errors:\n{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(crate::utils::error::ZosError::CommandFailed {
            command: format!("minizinc {}", args_str.join(" ")),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    println!("Phase 5 Complete: MiniZinc execution finished.");
```

1.  **Executable Location**: The `minizinc` executable is located within the `libminizinc` build directory, ensuring the correct version is used.
2.  **Command Construction**: The paths to the generated `ast_model.mzn` and `ast_data.dzn` files are passed as arguments to the `minizinc` executable.
3.  **Subprocess Execution**: The `subprocess::run_command` utility executes the `minizinc` command, capturing its standard output and standard error streams.
4.  **Error Handling**: The process includes checks for successful command execution, returning a detailed error if `minizinc` fails.

## Output Parsing Details

The relevant code snippet for parsing is:

```rust
    println!("\nPhase 6: Parsing MiniZinc Output...");
    let parsed_results = parse_minizinc_output(&String::from_utf8_lossy(&output.stdout))?;
    println!("Phase 6 Complete: MiniZinc Analysis Results ---");
    println!("Suggested Numerical Vector: {}", parsed_results.suggested_numerical_vector);
    println!("-----------------------------------");

// ... (parse_minizinc_output function definition) ...

fn parse_minizinc_output(output_str: &str) -> Result<MiniZincAnalysisResults> {
    let mut suggested_numerical_vector = 0;

    for line in output_str.lines() {
        if line.starts_with("suggested_numerical_vector =") {
            suggested_numerical_vector = line.split("=").nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        }
    }

    Ok(MiniZincAnalysisResults {
        suggested_numerical_vector,
    })
}
```

1.  **Output Capture**: The standard output from the `minizinc` execution is passed to the `parse_minizinc_output` function.
2.  **Value Extraction**: This function iterates through each line of the MiniZinc output. It specifically looks for the line starting with `"suggested_numerical_vector ="`.
3.  **Parsing**: The numerical value following the equals sign is extracted and parsed into an `i32`. This value represents the optimized numerical vector determined by the MiniZinc solver.

## Example Scenario (Continuing from previous steps)

Given our example Rust `add` function, which resulted in an `original_target_value` of `11` (for `crate_my_crate`), and the `modularity_prime` of `3`:

*   The MiniZinc solver would find the closest multiple of 3 to 11, which is 12.
*   The `minizinc` output would contain a line similar to: `suggested_numerical_vector = 12`.
*   The `parse_minizinc_output` function would successfully extract `12` as the `suggested_numerical_vector`.

This step is crucial as it bridges the gap between the symbolic representation of code and its numerical optimization, providing the concrete result of the MiniZinc analysis.

```
