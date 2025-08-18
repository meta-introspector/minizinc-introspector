# Rust Busy Box for Bootstrap: Design Document

## Date: August 16, 2025

## 1. Introduction
This document outlines the conceptual design for a "Rust Busy Box" tool, intended to replace a significant portion of the existing shell scripts in the `libminizinc` project. This initiative aligns with the project's "code oxidation" philosophy, aiming to consolidate build, test, and execution logic into a single, robust, and Rust-native executable. This will improve maintainability, error handling, and overall system reliability, moving towards a more self-aware and self-evolving system.

## 2. Scope: Shell Scripts to be Replaced (Initial Phase)
The initial phase of the Rust Busy Box will focus on replacing shell scripts that perform core bootstrap, build, test, and model execution functions. Utility scripts and those with highly specific `sed`/`awk` logic might be considered in later phases.

**Target Scripts for Replacement:**

### 2.1. Build & Setup
*   `build_gecode.sh`
*   `build_libminizinc.sh`
*   `build_minizinc_with_coverage.sh`
*   `generate_ffi_declarations.sh`
*   `setup_minizinc_solvers.sh`

### 2.2. Testing & Quality Assurance
*   `build_and_test.sh` (will be absorbed into a more general `test` command)
*   `build_test_c_rust.sh`
*   `qa_dzn_generation_verification.sh`
*   `reproduce_crash.sh` (will be absorbed into a `test` or `debug` command)
*   `reproduce_minizinc_ffi_bug.sh` (will be absorbed into a `test` or `debug` command)
*   `run_all_minizinc_tests.sh` (orchestration logic will be moved to Rust)
*   `run_rust_ffi_tests_for_coverage.sh`
*   `run_minizinc_test.sh` (core logic for running individual MiniZinc tests)
*   `run_v7_debug_tests.sh` (orchestration logic will be moved to Rust)
*   `test_rust_dzn_generator.sh` (will be absorbed into a `test` command)

### 2.3. Model Execution
*   `run_embedding_model_v6_test.sh`
*   `run_embedding_model_v6.sh`
*   `run_embedding_model_v7.sh`
*   `run_minizinc_minimal.sh`
*   `run_minizinc_test_driver.sh`

## 3. Conceptual Command-Line Interface (CLI)
The Rust Busy Box will be a single executable (e.g., `ragit-bootstrap`) with subcommands for different functionalities, similar to `cargo` or `git`.

**Proposed CLI Structure:**

```
ragit-bootstrap <COMMAND> [OPTIONS]
```

**Core Commands:**

*   **`ragit-bootstrap build`**: Handles the compilation of core project components.
    *   `ragit-bootstrap build all`: Builds everything (Gecode, libminizinc, FFI).
    *   `ragit-bootstrap build gecode`: Builds Gecode.
    *   `ragit-bootstrap build libminizinc`: Builds libminizinc.
    *   `ragit-bootstrap build ffi`: Builds the C++ FFI wrapper.
    *   `ragit-bootstrap build rust-ffi`: Builds the Rust FFI crate.
    *   `ragit-bootstrap build ffi-declarations`: Generates FFI header declarations.
    *   `ragit-bootstrap build solvers`: Sets up MiniZinc solver configurations.
    *   `ragit-bootstrap build coverage`: Builds libminizinc with coverage instrumentation.

*   **`ragit-bootstrap test`**: Executes various test suites.
    *   `ragit-bootstrap test all`: Runs all tests (C ABI, Rust FFI, MiniZinc models).
    *   `ragit-bootstrap test c-abi`: Runs the standalone C ABI test.
    *   `ragit-bootstrap test rust-ffi`: Runs the Rust FFI tests.
    *   `ragit-bootstrap test minizinc-models`: Runs the full MiniZinc model test suite.
    *   `ragit-bootstrap test dzn-gen <num_vec> <base_size>`: Generates and verifies DZN data.
    *   `ragit-bootstrap test coverage`: Runs Rust FFI tests for C++ coverage data generation.
    *   `ragit-bootstrap test dzn-gen-rust <num_vec>`: Tests the Rust DZN generator.

*   **`ragit-bootstrap run`**: Executes MiniZinc models.
    *   `ragit-bootstrap run embedding-v6 <main_model_version> ...`: Runs the v6 embedding model with proof tape.
    *   `ragit-bootstrap run embedding-v7 <main_model_version> ...`: Runs the v7 embedding model with dynamic DZN generation and proof tape.
    *   `ragit-bootstrap run minimal <main_model_version> ...`: Runs a MiniZinc model minimally (for quick debug).
    *   `ragit-bootstrap run driver <num_vec> <base_size>`: Runs the MiniZinc test driver (generates DZN, runs model).

*   **`ragit-bootstrap debug`**: Specific debugging utilities.
    *   `ragit-bootstrap debug reproduce-crash`: Attempts to reproduce a known FFI crash.
    *   `ragit-bootstrap debug reproduce-ffi-bug`: Attempts to reproduce a specific FFI bug.
    *   `ragit-bootstrap debug v7-tests`: Runs focused v7 debug tests.

*   **`ragit-bootstrap clean`**: Cleans build artifacts (with caution, adhering to "never delete" where possible).
    *   `ragit-bootstrap clean all`: Cleans all build directories.
    *   `ragit-bootstrap clean build`: Cleans the main build directory.
    *   `ragit-bootstrap clean coverage`: Cleans the coverage build directory.
    *   `ragit-bootstrap clean proof-tapes`: Cleans generated proof tapes.

## 4. Handling Environment Variables and Paths
The Rust Busy Box will manage environment variables and paths internally.
*   **Configuration:** Paths like `MINIZINC_PROJECT_ROOT`, `GECODE_BUILD_DIR`, `LIBMINIZINC_BUILD_DIR`, `MINIZINC_MODELS_DIR`, `MINIZINC_DATA_DIR`, `MINIZINC_USER_SOLVERS_DIR` will be read from a central configuration file (e.g., `ragit-bootstrap.toml` or derived from `.env` at runtime).
*   **Dynamic `LD_LIBRARY_PATH`:** The Rust tool will dynamically set `LD_LIBRARY_PATH` for subprocesses (e.g., `minizinc` executable, C ABI tests) as needed, ensuring correct runtime linking without relying on external shell exports.
*   **Relative Paths:** All internal path handling will prioritize relative paths where possible, improving portability.

## 5. Error Handling and User Feedback
Rust's robust error handling capabilities will be leveraged to provide clear, actionable error messages, replacing cryptic shell script failures.
*   **Structured Errors:** Errors will be structured (e.g., using `thiserror` crate) to provide context and suggest solutions.
*   **Verbose Output:** Options for verbose output (`-v`, `--verbose`) will be implemented to aid debugging.
*   **Progress Indicators:** Provide better feedback during long-running operations (e.g., builds).

## 6. Integration with External Tools
The Rust Busy Box will orchestrate calls to external binaries like `cmake`, `make`, `g++`, `minizinc`, `llvm-profdata`, `llvm-cov`, `cargo`. It will manage their execution, capture output, and handle exit codes.

## 7. Adherence to Monotonic Epic Idea
The Rust Busy Box will be designed to support the "add-only, never edit" philosophy.
*   **Supersession:** New functionalities will be added as new subcommands or modules, superseding old shell scripts.
*   **Immutable Artifacts:** It will continue to generate immutable artifacts like proof tapes.
*   **Controlled Clean:** The `clean` command will be implemented with caution, respecting the "never delete" principle where historical data is concerned (e.g., only cleaning temporary build artifacts, not committed data).

## 8. Future Phases (Beyond Initial Bootstrap)
*   **Code Analysis Integration:** Integrate code parsing (Rust AST, MiniZinc AST) directly into the Rust tool for semantic feature extraction (Phase 1 of "big idea").
*   **LLM Orchestration:** Orchestrate LLM calls for semantic summarization and interpretation.
*   **Numerical Embedding Analysis:** Implement Rust modules for comparing numerical embeddings and identifying duplicates.

This design provides a roadmap for transitioning the project's operational core to a more robust, Rust-native foundation, directly enabling the "big idea" and the project's vision of computational self-awareness.
