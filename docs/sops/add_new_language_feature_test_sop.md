# Standard Operating Procedure: Adding New MiniZinc Language Feature Tests for Code Coverage

## 1. Purpose
This SOP outlines the process for systematically adding new tests to the `libminizinc` project to increase code coverage for specific MiniZinc language features. By following this procedure, developers can ensure that new functionalities are adequately tested and that the overall codebase quality is improved.

## 2. Scope
This SOP applies to the process of creating and integrating new MiniZinc models and corresponding Rust Foreign Function Interface (FFI) tests within the `libminizinc` project. It focuses on extending test coverage for previously untested or under-tested MiniZinc language constructs.

## 3. Prerequisites
Before commencing this procedure, ensure the following:
*   A functional `libminizinc` development environment is set up.
*   `llvm-cov` and `llvm-profdata` tools are installed and accessible in your system's PATH.
*   Basic understanding of MiniZinc language syntax.
*   Familiarity with Rust FFI testing patterns within the `libminizinc` project.
*   The `build.rs` script in `tools/minizinc_ffi/` is correctly configured to link against the `build_coverage` directory.
*   The parser has been regenerated at least once after any modifications to `lib/parser.yxx` or `lib/lexer.lxx`.

## 4. Procedure

### Phase 1: Identify and Prepare New Language Feature Test

1.  **Identify Untested Language Feature**:
    Review the MiniZinc grammar (`lib/parser.yxx`) and existing coverage reports (`html_llvm_coverage_report_per_test/index.html` or `coverage_reports_new_content/html_report/index.html`) to pinpoint a MiniZinc language feature that has low or no coverage.
    *   *Example*: A specific operator (e.g., `/
`), a control flow statement (`if-then-else`), or a data structure (`array`, `set`).

2.  **Create MiniZinc Model (`.mzn`)**:
    *   Write a minimal MiniZinc model (`.mzn` file) that exclusively uses and demonstrates the identified language feature.
    *   Save this file in the `tests/` directory of the `libminizinc` project (e.g., `tests/my_new_feature_test.mzn`).
    *   *Example*: For testing the `/
` operator:
        ```minizinc
        var bool: a;
        var bool: b;
        constraint a /
 b;
        solve satisfy;
        ```

### Phase 2: Create and Integrate Rust FFI Test

1.  **Create Rust FFI Test File**:
    *   Navigate to `tools/minizinc_ffi/src/tests/tests/`.
    *   Create a new Rust file following the naming convention (e.g., `test_XX_my_new_feature.rs`, where `XX` is the next sequential number).
    *   Add the basic test structure, including `#[cfg(test)]`, `mod`, `use`, and `#[test]` attributes.
    *   *Example (`tools/minizinc_ffi/src/tests/tests/test_07_boolean_and.rs`)*:
        ```rust
        #[cfg(test)]
        mod test_07_boolean_and {
            use crate::tests::tests::GLOBAL_MINIZINC_ENV;

            #[test]
            fn test_07_parse_boolean_and() {
                let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
                // Use include_str! to embed the MiniZinc model directly
                let model_code = include_str!("../../../../../tests/boolean_and_test.mzn");
                let model = env.parse_string(model_code);
                assert!(model.is_ok()); // Assert that parsing was successful
                println!("Test 07: Parsed boolean AND feature model.");
            }
        }
        ```

2.  **Integrate New Test Module**:
    *   Open `tools/minizinc_ffi/src/tests/mod.rs`.
    *   Add a `mod` declaration for the new test file within the `#[cfg(test)] mod tests { ... }` block.
    *   *Example*:
        ```rust
        mod test_06_new_feature;
        mod test_07_boolean_and; // Add this line
        ```

### Phase 3: Build, Run, and Analyze Coverage

1.  **Rebuild MiniZinc with Coverage Instrumentation**:
    *   Execute the C++ build script to ensure all changes, including the new MiniZinc model and any parser modifications, are compiled with coverage instrumentation.
    *   `./build_minizinc_with_coverage.sh`

2.  **Run Rust FFI Tests**:
    *   Execute the Rust test suite. This will run all tests, including the newly added one, and generate raw LLVM profile data (`.profraw` files).
    *   `./scripts/run_rust_ffi_tests_for_coverage.sh`

3.  **Merge Raw Profile Data**:
    *   Combine all generated `.profraw` files into a single `.profdata` file. This is crucial for accurate coverage reporting.
    *   `llvm-profdata merge -sparse ./tools/minizinc_ffi/test-*.profraw -o merged.profdata`

4.  **Generate Coverage Reports**:
    *   **Text Summary**: Generate a high-level text summary of the overall code coverage.
        `llvm-cov report -ignore-filename-regex='.*(test|thirdparty).*' /data/data/com.termux/files/home/storage/github/libminizinc/build_coverage/libminizinc_c_wrapper.so -instr-profile=merged.profdata`
    *   **HTML Report (for visual inspection)**: Generate a detailed HTML report for visual analysis of line-by-line coverage.
        `llvm-cov show -format=html -output-dir=coverage_reports_new_content/html_report -ignore-filename-regex='.*(test|thirdparty).*' /data/data/com.termux/files/home/storage/github/libminizinc/build_coverage/libminizinc_c_wrapper.so -instr-profile=merged.profdata`
        *   *Note*: You can also generate HTML reports for specific files (e.g., `lib/parser.cpp`, `lib/parser.yxx`'s generated C++ files, or `tools/minizinc_c_wrapper_refactored/minizinc_parse_string_only.cpp`) by adding their paths to the `llvm-cov show` command.

5.  **Analyze Coverage Results**:
    *   Review the text summary for overall changes in coverage percentages.
    *   Open the generated HTML reports in a web browser. Navigate to the relevant source files (e.g., `lib/parser.cpp`, `lib/parser.yxx`'s generated C++ files, or `tools/minizinc_c_wrapper_refactored/minizinc_parse_string_only.cpp`) and visually inspect for newly covered lines (highlighted in green) and increased execution counts. Pay close attention to the code paths related to the new language feature.

## 5. Tools
*   MiniZinc Compiler (C++ codebase)
*   Rust Compiler (`cargo`)
*   `llvm-cov`
*   `llvm-profdata`
*   `git` (for version control)
*   Text Editor

## 6. Expected Outcome
*   Increased code coverage for the targeted MiniZinc language feature.
*   A passing Rust FFI test for the new feature.
*   Updated coverage reports reflecting the changes.
*   Improved understanding of the MiniZinc parser's behavior and coverage.
*   A more robust and thoroughly tested `libminizinc` codebase.
