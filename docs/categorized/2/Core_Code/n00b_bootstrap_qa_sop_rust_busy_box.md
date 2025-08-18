# N00b Bootstrap and QA SOP: Leveraging the Rust Busy Box

## Date: August 16, 2025

## 1. Introduction
This Standard Operating Procedure (SOP) guides new developers ("n00bs") through the process of bootstrapping and performing basic Quality Assurance (QA) within the `libminizinc` project, leveraging the new Rust Busy Box (`ragit-bootstrap`). This SOP supersedes previous shell-script-based instructions, providing a more streamlined and robust experience.

## 2. Prerequisites
*   Basic understanding of Rust and Cargo.
*   A working Rust toolchain installed.
*   The `libminizinc` project cloned to your local machine.

## 3. Bootstrapping the System with `ragit-bootstrap`

### 3.1. Initial Build of the Busy Box
Before you can use `ragit-bootstrap`, you need to build it.
*   Navigate to the project root: `cd /data/data/com.termux/files/home/storage/github/libminizinc`
*   Build the `ragit-bootstrap` executable:
    ```bash
    cargo build --release -p ragit-bootstrap
    ```
    This will compile the busy box and place the executable in `target/release/ragit-bootstrap`.

### 3.2. Full System Bootstrap
The `ragit-bootstrap` tool provides a single command to build all core components and run initial tests.
*   Execute the full bootstrap:
    ```bash
    ./target/release/ragit-bootstrap build all
    ./target/release/ragit-bootstrap test all
    ```
    *   **`build all`**: This command will compile Gecode, `libminizinc`, the C++ FFI wrapper, and the Rust FFI crate. It will also generate FFI declarations and set up MiniZinc solver configurations.
    *   **`test all`**: This command will execute the C ABI tests, Rust FFI tests, and the full MiniZinc model test suite.

### 3.3. Verifying the Bootstrap
*   **Check Build Output:** Look for "Build complete" messages and no critical errors during the `build all` step.
*   **Check Test Output:** Ensure all tests pass during the `test all` step. Look for "All tests completed successfully" or similar messages.
*   **Verify MiniZinc Executable:**
    ```bash
    ./target/release/ragit-bootstrap run minimal v6 v1 v1 v1 v1 v1
    ```
    This should run a minimal MiniZinc model and output its results without errors.

## 4. Basic Quality Assurance (QA) Steps

### 4.1. Running Specific Test Suites
The `ragit-bootstrap test` command allows you to run specific test categories.
*   **Rust FFI Tests:**
    ```bash
    ./target/release/ragit-bootstrap test rust-ffi
    ```
*   **MiniZinc Model Tests:**
    ```bash
    ./target/release/ragit-bootstrap test minizinc-models
    ```
*   **C ABI Test:**
    ```bash
    ./target/release/ragit-bootstrap test c-abi
    ```

### 4.2. Generating and Verifying DZN Data
The `ragit-bootstrap` tool can also generate and verify MiniZinc DZN data files.
*   **Generate and Verify:**
    ```bash
    ./target/release/ragit-bootstrap test dzn-gen 10 100
    ```
    This will generate a DZN file with `num_vec=10` and `base_size=100`, and then verify its correctness.

### 4.3. Running Embedding Models
You can execute specific MiniZinc embedding models using the `run` command.
*   **Run v7 Embedding Model (with dynamic DZN generation):**
    ```bash
    ./target/release/ragit-bootstrap run embedding-v7 v6 v1 v1 v1 v1 10
    ```
    This will dynamically generate vector parameters and run the `v7` embedding model, creating a proof tape.

## 5. Debugging with `ragit-bootstrap`

The `ragit-bootstrap debug` command provides utilities for reproducing and investigating issues.
*   **Reproduce FFI Crash:**
    ```bash
    ./target/release/ragit-bootstrap debug reproduce-crash
    ```
*   **Reproduce Specific FFI Bug:**
    ```bash
    ./target/release/ragit-bootstrap debug reproduce-ffi-bug
    ```

## 6. Adherence to Monotonic Epic Idea
Remember that all development in this project adheres to the "add-only, never edit" philosophy. When making changes, create new files or new versions of existing components, and use `ragit-bootstrap` to integrate them.

## 7. Troubleshooting
*   **`ragit-bootstrap` not found:** Ensure you have built the executable (`cargo build --release -p ragit-bootstrap`) and are running it from the correct path (`./target/release/ragit-bootstrap`).
*   **Build failures:** Check the output for specific error messages from `cmake`, `make`, or `cargo`.
*   **Test failures:** Review the output from the `test` commands for details on why tests failed.
*   **MiniZinc errors:** If MiniZinc models fail to run, use `ragit-bootstrap run minimal` for quick debugging, or inspect the proof tape directories for full logs.

This SOP provides a streamlined approach to getting started with the `libminizinc` project, leveraging the power and robustness of the Rust Busy Box.
