# MiniZinc Model Performance Analysis and Debugging Report

## Date: August 14, 2025

## 1. Introduction

This report details the process of debugging and analyzing the performance of MiniZinc models within the `libminizinc` project, specifically focusing on the `run_embedding_model_v6.sh` script and its interaction with various data configurations. The primary goal was to understand unexpected performance results for models with varying `num_vec` (number of vectors) parameters and to establish a robust testing framework.

## 2. Problem Statement: The "Performance Fluke"

Initial performance tests using `scripts/run_all_v6_tests.sh` showed counter-intuitive results: MiniZinc models with very large `num_vec` values (e.g., 10000) appeared to run significantly faster than those with smaller `num_vec` values (e.g., 2, 7). This suggested a fundamental misunderstanding of the model's execution or an underlying issue causing premature termination.

## 3. Debugging Process

The debugging process involved a systematic investigation of the MiniZinc execution pipeline and data handling:

### 3.1. Initial Investigation: Empty Files and Missing Definitions

*   **Observation:** The `run_embedding_model_v6.sh` script was failing with "undefined identifier" errors for core parameters (`n`, `d`, `P`, `KAPPA_SCALE`, `PARTITION_SCALE`) when using `example_core_params_nvX.dzn` files.
*   **Finding:** Inspection revealed that `minizinc_models/embedding_sphere_final_v6.mzn` and several `example_core_params_nvX.dzn` files were empty or incomplete.
*   **Action:**
    *   `embedding_sphere_final_v6.mzn` was populated with its intended `include` statements.
    *   `example_core_params_v1.dzn` was restored from Git history and then manually populated with placeholder values for all required core parameters (`n`, `d`, `P`, `KAPPA_SCALE`, `PARTITION_SCALE`, `num_vec`).
    *   All `example_core_params_nvX.dzn` files were updated to include all core parameters, with `num_vec` set to their respective `X` value.

### 3.2. DZN Array Formatting Issues

*   **Observation:** After fixing the core parameter definitions, the models started failing with "Index set mismatch" errors, specifically for `alpha_coeff` in `example_vector_params_v1.dzn`.
*   **Finding:** The Rust program used to generate DZN data was using `{:?}` for array formatting, which produced output incompatible with MiniZinc's strict DZN array syntax (e.g., `[0.0, 0.0,]` with a trailing comma, or incorrect spacing).
*   **Action:** The Rust code in `minizinc_data_generator_rs/src/main.rs` was modified to manually format the arrays, ensuring strict DZN compatibility (e.g., `[0.0, 0.0]`).

### 3.3. MiniZinc `--output-dzn` Misunderstanding

*   **Observation:** Attempts to use `minizinc --output-dzn` to generate DZN files from a MiniZinc model resulted in unexpected output (`----------`) or errors.
*   **Finding:** The `minizinc --output-dzn` option is not supported by the installed MiniZinc version. The correct option for DZN output is `--output-mode dzn`, which is intended for solution output, not for generating data files from a model's `output` statement.
*   **Action:** This confirmed that a dedicated program (like the Rust generator) was necessary for DZN file generation.

### 3.4. Proof Tape Logging and Debugging Output

*   **Observation:** `full_output.log` files within `proof_tapes/` were often empty or inaccessible, hindering debugging.
*   **Finding:** The `run_embedding_model_v6.sh` script's redirection of MiniZinc output was not consistently capturing all information, and there were issues with file persistence/access in the Termux environment.
*   **Action:**
    *   The `.gitignore` entry for `proof_tapes/` was temporarily removed to allow direct inspection.
    *   `run_embedding_model_v7.sh` was created as a new version of the script.
    *   The `run_embedding_model_v7.sh` script was modified to use `tee` to simultaneously display MiniZinc output and write it to `full_output.log`, ensuring the log file is populated.
    *   Debugging `ls -l` commands were added to `run_embedding_model_v7.sh` to verify file creation and content.

### 3.5. Rust Program `ParseIntError`

*   **Observation:** The Rust `minizinc_data_generator_rs` program was panicking with `ParseIntError` when receiving `CORE_V` values like `"nv2"`.
*   **Finding:** The Rust program expected a pure integer, but the shell script was passing the full version string (e.g., "nv2").
*   **Action:** `run_v7_debug_tests.sh` was modified to extract only the numerical part from `CORE_V` (e.g., "2" from "nv2") before passing it to the Rust program.

## 4. Solution Implemented

To address the identified issues and establish a robust testing framework, the following solutions were implemented:

*   **Dynamic DZN Data Generation (Rust):**
    *   A new Rust binary crate, `tools/minizinc_data_generator_rs`, was created.
    *   This program takes `num_vec` as a command-line argument and generates the `alpha_coeff`, `beta_coeff`, `m_idx`, `n_idx`, and `t_idx` arrays in correct DZN format.
    *   This eliminates manual DZN file creation and ensures correct array sizing.
*   **Rust-based Test Runner:**
    *   A new Rust binary crate, `tools/minizinc_test_runner_rs`, was created to serve as the primary test driver.
    *   This program orchestrates the execution of MiniZinc models, calls the `minizinc_data_generator_rs` to prepare data, runs the MiniZinc solver, captures output, measures execution time, and presents sorted results.
    *   This replaces the complex and error-prone shell scripts for test orchestration.
*   **Versioned Scripting:**
    *   `scripts/run_embedding_model_v7.sh` was introduced as a new version of the model execution script, incorporating the Rust DZN generator.
    *   `scripts/run_v7_debug_tests.sh` was created as a focused test driver for debugging, calling the Rust test runner.
*   **Clean-up:** Obsolete files and scripts (`generate_vector_params.mzn`, `run_all_v6_tests.sh`, `example_vector_params_v1.dzn`, `v6_test_results.txt`, `v7_debug_test_results.txt`) were removed.
*   **Git Ignore:** `.gitignore` files were added to the new Rust projects to prevent `target/` directories from being committed.

## 5. Performance Findings: `num_vec` vs. Execution Time

After implementing the solutions, accurate performance data was obtained. The results clearly demonstrate that as `num_vec` increases, the MiniZinc model's execution time generally increases significantly, as expected due to the larger problem size and search space.

Here are the final sorted results (fastest to slowest):

```
0.080316094 v6 nv1 v1 v1 v1 1
0.08641625 v6 v1 v1 v1 v1 1
0.104021093 v6 nv2 v1 v1 v1 2
0.105738281 v6 nv7 v1 v1 v1 7
0.109313646 v6 nv5 v1 v1 v1 5
0.114788073 v6 nv3 v1 v1 v1 3
0.120792917 v6 nv11 v1 v1 v1 11
0.123730052 v6 nv10 v1 v1 v1 10
0.137260573 v6 nv13 v1 v1 v1 13
0.14091724 v6 nv17 v1 v1 v1 17
0.222104948 v6 nv100 v1 v1 v1 100
1.385801146 v6 nv1000 v1 v1 v1 1000
15.503735098 v6 nv10000 v1 v1 v1 10000
```

The initial "performance fluke" was indeed due to MiniZinc failing early on larger problems before the actual solving phase, leading to deceptively fast reported times. With the correct data generation, the true computational cost of larger `num_vec` values is now evident.

## 6. Standard Operating Procedures (SOPs) for Noobs

To make this process easier to use and understand for new contributors ("noobs"), the following SOPs are recommended:

### SOP 1: Running MiniZinc Performance Tests

1.  **Prerequisites:**
    *   Ensure Rust and Cargo are installed.
    *   Ensure MiniZinc is installed and the `minizinc` executable is in your `PATH` or accessible via the `LIBMINIZINC_BUILD_DIR` environment variable (defined in `.env`).
2.  **Navigate to Project Root:** Open your terminal and navigate to the `libminizinc` project root directory.
3.  **Build Rust Components:**
    ```bash
    cd tools/minizinc_data_generator_rs/minizinc_data_generator_rs
    cargo build --release
    cd ../../minizinc_test_runner_rs/minizinc_test_runner_rs
    cargo build --release
    cd ../../.. # Go back to project root
    ```
4.  **Run Tests:**
    *   For a focused debug run (as used in this report):
        ```bash
        bash scripts/run_v7_debug_tests.sh
        ```
    *   To run a comprehensive set of tests (if a `run_all_v7_tests.sh` is created later):
        ```bash
        bash scripts/run_all_v7_tests.sh
        ```
5.  **Analyze Results:** The script will print the sorted performance results directly to the console. Detailed logs for each run are stored in `proof_tapes/` directories (named by timestamp).

### SOP 2: Understanding MiniZinc Data Files (`.dzn`)

*   **Purpose:** `.dzn` files provide input data (parameters) to MiniZinc models (`.mzn` files).
*   **Format:** Variables are assigned values using `variable_name = value;`. Arrays are defined using `[elem1, elem2, ...]`.
*   **Dynamic Generation:** For arrays whose size depends on a parameter (like `num_vec`), use the `minizinc_data_generator_rs` Rust program. It ensures correct DZN syntax and array sizing.
*   **Example:**
    ```dzn
    % example_core_params_nvX.dzn
    n = 7;
    d = 8;
    P = 5;
    KAPPA_SCALE = 100;
    PARTITION_SCALE = 100;
    num_vec = X; % Where X is the specific number of vectors for this file
    ```

### SOP 3: Debugging MiniZinc Model Execution

1.  **Check Script Output:** Always review the console output of the test runner script for any errors or warnings.
2.  **Inspect Proof Tapes:** For detailed MiniZinc output, navigate to the `proof_tapes/` directory corresponding to the timestamp of the failed run.
    *   `full_output.log`: Contains the complete stdout and stderr from the MiniZinc solver.
    *   `error.log`: Contains a summary of errors if the model failed.
    *   Other files: Copies of the model and data files used for that specific run.
3.  **Analyze MiniZinc Output:** Look for:
    *   `Parsing file(s) ... done parsing`: Indicates successful model and data parsing.
    *   `Flattening ... Done`: Indicates successful model flattening.
    *   `% SOLVING PHASE`: Indicates the solver is actively searching for solutions. If this is missing, the model might be unsatisfiable or trivially solved.
    *   `=====UNSATISFIABLE=====`, `=====SATISFIABLE=====`, `==========`: Solution status messages.

## 7. Future Improvements

*   **Comprehensive Test Suite:** Expand `run_v7_debug_tests.sh` into a `run_all_v7_tests.sh` that covers a wider range of parameter combinations and model versions.
*   **Automated DZN Generation for All Parameters:** Extend the Rust data generator to handle other varying parameters (e.g., `n`, `d`, `P`) if needed, to further automate data file creation.
*   **Graphical Visualization:** Explore tools for visualizing the performance data (e.g., plotting `num_vec` vs. time).
*   **CI/CD Integration:** Integrate these tests into a continuous integration pipeline to automatically track performance changes.

---
