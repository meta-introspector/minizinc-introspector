# Rust Busy Box: Implementation Outline (Conceptual)

## Date: August 16, 2025

## 1. Introduction
This document outlines the conceptual implementation structure for the Rust Busy Box, detailing the modules, key functions, and their responsibilities in replacing the shell script functionalities as defined in the [Rust Busy Box for Bootstrap: Design Document](rust_busy_box_bootstrap_design.md).

## 2. Project Structure (Conceptual)

```
ragit-bootstrap/
├── src/
│   ├── main.rs                 # CLI entry point, command parsing
│   ├── commands/               # Module for each top-level command (build, test, run, debug, clean)
│   │   ├── mod.rs
│   │   ├── build.rs
│   │   ├── test.rs
│   │   ├── run.rs
│   │   ├── debug.rs
│   │   └── clean.rs
│   ├── utils/                  # General utility functions (path handling, subprocess execution, error handling)
│   │   ├── mod.rs
│   │   ├── paths.rs            # Path resolution, environment variable management
│   │   ├── subprocess.rs       # Wrapper for running external commands
│   │   ├── minizinc_cmd.rs     # MiniZinc command construction and execution
│   │   ├── proof_tape.rs       # Proof tape generation logic
│   │   └── error.rs            # Custom error types and handling
│   ├── config.rs               # Configuration loading (e.g., from .env or ragit-bootstrap.toml)
│   ├── lib.rs                  # Crate root, re-exports
│   └── types.rs                # Common data structures (e.g., version vectors)
├── Cargo.toml
├── .env.example
└── ragit-bootstrap.toml.example # Example configuration file
```

## 3. Core Module Responsibilities and Key Functions (Conceptual)

### 3.1. `src/main.rs`
*   **Responsibility:** Parses command-line arguments, dispatches to appropriate subcommands.
*   **Key Functions:**
    *   `main()`: Entry point, uses `clap` or similar for CLI parsing.
    *   `dispatch_command()`: Matches subcommand and calls its handler.

### 3.2. `src/commands/build.rs`
*   **Responsibility:** Implements the `ragit-bootstrap build` subcommand and its sub-subcommands. Orchestrates compilation processes.
*   **Key Functions:**
    *   `handle_build_command()`: Main handler for `build` subcommand.
    *   `build_gecode()`: Replaces `build_gecode.sh`. Calls `cmake` and `make`.
    *   `build_libminizinc()`: Replaces `build_libminizinc.sh`. Calls `cmake` and `make`.
    *   `build_minizinc_with_coverage()`: Replaces `build_minizinc_with_coverage.sh`. Calls `cmake` with coverage flags.
    *   `generate_ffi_declarations()`: Replaces `generate_ffi_declarations.sh`. Reads `.h` files, writes consolidated header.
    *   `setup_minizinc_solvers()`: Replaces `setup_minizinc_solvers.sh`. Writes `.msc` files.
    *   `build_ffi_wrapper()`: Builds the C++ FFI wrapper.
    *   `build_rust_ffi_crate()`: Builds the Rust FFI crate.

### 3.3. `src/commands/test.rs`
*   **Responsibility:** Implements the `ragit-bootstrap test` subcommand. Orchestrates test execution and result reporting.
*   **Key Functions:**
    *   `handle_test_command()`: Main handler for `test` subcommand.
    *   `test_c_abi()`: Replaces `build_test_c_rust.sh` (partially). Generates and compiles C++ test, runs it.
    *   `test_rust_ffi()`: Replaces `build_and_test.sh` (partially) and `run_rust_ffi_tests_for_coverage.sh` (partially). Runs `cargo test`.
    *   `test_minizinc_models()`: Replaces `run_all_minizinc_tests.sh`. Orchestrates MiniZinc model tests.
    *   `test_dzn_generation()`: Replaces `qa_dzn_generation_verification.sh`. Calls MiniZinc for DZN generation and verification.
    *   `test_dzn_gen_rust()`: Replaces `test_rust_dzn_generator.sh`. Calls `minizinc_data_generator_rs`.
    *   `test_coverage()`: Replaces `run_rust_ffi_tests_for_coverage.sh` (partially). Runs `cargo test` with profiling.

### 3.4. `src/commands/run.rs`
*   **Responsibility:** Implements the `ragit-bootstrap run` subcommand. Executes MiniZinc models.
*   **Key Functions:**
    *   `handle_run_command()`: Main handler for `run` subcommand.
    *   `run_embedding_v6()`: Replaces `run_embedding_model_v6.sh` and `run_embedding_model_v6_test.sh`. Uses `utils::minizinc_cmd` and `utils::proof_tape`.
    *   `run_embedding_v7()`: Replaces `run_embedding_model_v7.sh`. Integrates Rust DZN generator.
    *   `run_minimal_mzn()`: Replaces `run_minizinc_minimal.sh`.
    *   `run_test_driver()`: Replaces `run_minizinc_test_driver.sh`.

### 3.5. `src/commands/debug.rs`
*   **Responsibility:** Implements the `ragit-bootstrap debug` subcommand. Provides specific debugging utilities.
*   **Key Functions:**
    *   `handle_debug_command()`: Main handler for `debug` subcommand.
    *   `reproduce_crash()`: Replaces `reproduce_crash.sh`.
    *   `reproduce_ffi_bug()`: Replaces `reproduce_minizinc_ffi_bug.sh`.
    *   `run_v7_debug_tests()`: Replaces `run_v7_debug_tests.sh`.

### 3.6. `src/commands/clean.rs`
*   **Responsibility:** Implements the `ragit-bootstrap clean` subcommand. Manages cleaning of build artifacts.
*   **Key Functions:**
    *   `handle_clean_command()`: Main handler for `clean` subcommand.
    *   `clean_build_dir()`: Cleans specified build directories.
    *   `clean_proof_tapes()`: Cleans proof tape directories.

### 3.7. `src/utils/paths.rs`
*   **Responsibility:** Centralized path resolution and environment variable management.
*   **Key Functions:**
    *   `resolve_project_root()`: Determines the project root.
    *   `get_build_dir()`: Returns path to main build directory.
    *   `get_gecode_build_dir()`: Returns path to Gecode build directory.
    *   `get_minizinc_models_dir()`: Returns path to MiniZinc models.
    *   `set_ld_library_path()`: Dynamically sets `LD_LIBRARY_PATH` for subprocesses.

### 3.8. `src/utils/subprocess.rs`
*   **Responsibility:** Generic wrapper for executing external commands, capturing output, and handling errors.
*   **Key Functions:**
    *   `run_command()`: Executes a command, returns stdout/stderr/exit code.
    *   `run_command_with_env()`: Executes a command with custom environment variables.

### 3.9. `src/utils/minizinc_cmd.rs`
*   **Responsibility:** Encapsulates MiniZinc command construction and execution logic.
*   **Key Functions:**
    *   `build_minizinc_command()`: Constructs the `minizinc` command string with flags and file paths.
    *   `execute_minizinc()`: Executes the constructed MiniZinc command.

### 3.10. `src/utils/proof_tape.rs`
*   **Responsibility:** Encapsulates proof tape generation logic.
*   **Key Functions:**
    *   `generate_proof_tape_dir()`: Creates timestamped directory.
    *   `record_version_vector()`: Writes version info to file.
    *   `copy_files_to_proof_tape()`: Copies specified files.
    *   `capture_minizinc_output()`: Redirects MiniZinc output to proof tape logs.

### 3.11. `src/utils/error.rs`
*   **Responsibility:** Defines custom error types for the busy box.
*   **Key Functions:**
    *   Custom `Error` enum (using `thiserror`).
    *   `Result` type alias.

### 3.12. `src/config.rs`
*   **Responsibility:** Loads configuration from a file (e.g., `ragit-bootstrap.toml`).
*   **Key Functions:**
    *   `load_config()`: Reads and parses configuration.

## 4. Adherence to Monotonic Epic Idea
*   The implementation will prioritize adding new Rust code.
*   Existing shell scripts will be superseded, not directly modified (though they will remain in the repository for historical context).
*   The Rust tool itself will be designed for extensibility, allowing new subcommands and functionalities to be added without altering existing ones.

This outline provides a clear roadmap for the conceptual implementation of the Rust Busy Box, laying the groundwork for a more robust and Rust-native bootstrap system.
