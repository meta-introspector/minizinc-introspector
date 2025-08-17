# Summary of Shell Scripts in libminizinc Project

## Date: August 16, 2025

## Introduction
This document provides a high-level summary of the shell scripts found within the `libminizinc` project. These scripts play a crucial role in automating various development, build, test, and execution workflows, directly supporting the project's ambitious goals of computational self-awareness and semantic embedding.

## Categories of Shell Scripts

### 1. Build & Setup Scripts
These scripts are responsible for configuring and compiling the core components of the `libminizinc` project, including its dependencies and FFI-related elements. They ensure a consistent and reproducible build environment.

*   **`build_gecode.sh`**: Compiles the Gecode constraint programming library, a key solver backend for MiniZinc.
*   **`build_libminizinc.sh`**: Builds the core `libminizinc` C++ library, which is the foundation for MiniZinc functionality.
*   **`build_minizinc_with_coverage.sh`**: Builds `libminizinc` with C++ code coverage instrumentation, enabling self-analysis of code quality.
*   **`generate_ffi_declarations.sh`**: Automates the generation of the C header file that defines the interface between Rust FFI and the MiniZinc C++ library, ensuring FFI consistency and modularity.
*   **`setup_minizinc_solvers.sh`**: Configures MiniZinc to discover and use external solvers (e.g., Gecode) by creating `.msc` files.

### 2. Testing & Quality Assurance (QA) Scripts
This category includes scripts dedicated to verifying the correctness, stability, and performance of various project components, particularly the FFI and MiniZinc models.

*   **`build_and_test.sh`**: A general script to build the C++ wrapper and run Cargo tests, often used for crash reproduction.
*   **`build_test_c_rust.sh`**: A comprehensive script that builds the C++ wrapper, creates and compiles a standalone C++ test to verify the C ABI of the FFI, and then runs the Rust FFI tests.
*   **`qa_dzn_generation_verification.sh`**: Automates the generation and verification of MiniZinc DZN data files, ensuring data quality for model inputs.
*   **`reproduce_crash.sh`**: Specifically designed to reproduce and capture a crash in the MiniZinc FFI, aiding in debugging and stability efforts.
*   **`reproduce_minizinc_ffi_bug.sh`**: Focuses on reproducing a specific MiniZinc FFI bug, setting up the environment and running Rust tests to capture the issue.
*   **`run_all_minizinc_tests.sh`**: Orchestrates the execution of the entire MiniZinc model test suite, collecting and reporting test durations.
*   **`run_rust_ffi_tests_for_coverage.sh`**: Runs Rust FFI tests and generates raw LLVM profile data for C++ code coverage, as well as Rust code coverage reports.
*   **`run_minizinc_test.sh`**: A generic test runner for individual MiniZinc models, capturing output and measuring execution time.
*   **`run_v7_debug_tests.sh`**: Orchestrates focused debug test runs of MiniZinc models using `run_embedding_model_v7.sh`, often driven by a Rust test runner.
*   **`test_rust_dzn_generator.sh`**: A simple test to verify the functionality of the Rust DZN generator, a key component for dynamic data generation.

### 3. Model Execution Scripts
These scripts are responsible for running the core MiniZinc models, especially those related to the semantic embedding process, often with advanced features like versioning and proof tape generation.

*   **`run_embedding_model_v6_test.sh`**: A test version of the `v6` embedding model runner, featuring versioned modules and automatic proof tape generation for reproducibility.
*   **`run_embedding_model_v6.sh`**: The primary production script for running the `v6` embedding model, similar to its test counterpart but intended for regular use.
*   **`run_embedding_model_v7.sh`**: An advanced embedding model runner that dynamically generates MiniZinc DZN data using a Rust program, showcasing a higher level of automation in the data pipeline.
*   **`run_minizinc_minimal.sh`**: A stripped-down script for quick MiniZinc model execution and direct error output, useful for rapid debugging without proof tape overhead.
*   **`run_minizinc_test_driver.sh`**: A comprehensive script that generates MiniZinc data files and then runs the main MiniZinc model with this data, acting as an end-to-end test for the MiniZinc pipeline.

### 4. Editor & Utility Scripts
These scripts provide general utility functions, often supporting development practices or specific editor integrations.

*   **`generate_pub_uses.sh`**: Automates the generation of `pub mod` and `pub use` statements for Rust `lib.rs` files, enforcing the "one declaration per file" principle.
*   **`replace_abs_path_prefix.sh`**: Replaces hardcoded absolute path prefixes with relative paths within files, enhancing portability.
*   **`template_ffi_replace.sh`**: A template for highly specific text replacement, intended for FFI function declaration changes (noted as fragile and against the "add-only" philosophy).
*   **`ltmain.sh`**: A standard GNU Libtool script, part of the underlying build infrastructure for shared libraries (located in `vendor/`).

## Overall Contribution to the "Big Idea"

The collection of shell scripts forms the operational backbone of the `libminizinc` project, directly supporting the "big idea" of numerical representation and computational self-awareness in several critical ways:

*   **Automation of Core Workflows:** They automate the entire pipeline from building dependencies (Gecode, libminizinc), to compiling the FFI, running complex MiniZinc embedding models, and generating data for these models. This automation is essential for the scalability and reproducibility required by the "big idea."
*   **Quality Assurance and Reliability:** A significant portion of the scripts is dedicated to testing, debugging, and coverage analysis. This rigorous QA ensures the stability of the FFI (the bridge between Rust and MiniZinc) and the correctness of the MiniZinc models, which are paramount for reliable semantic embedding.
*   **Reproducibility and Traceability:** Scripts like `run_embedding_model_v6.sh` and `v7.sh` with their "proof tape" generation are fundamental to the project's commitment to perfect traceability. This allows for the precise analysis of how numerical representations evolve over time.
*   **Self-Introspection and Self-Evolution:** Scripts that build with coverage, generate DZN data from Rust, or orchestrate debug tests contribute directly to the project's ability to analyze and improve itself. This is a practical manifestation of computational self-awareness.
*   **Enforcement of Philosophies:** Scripts like `build_minizinc_with_coverage.sh` and `generate_pub_uses.sh` actively enforce core development philosophies such as "add-only, never edit" and "one declaration per file," which in turn make the codebase more amenable to semantic analysis.

In essence, these shell scripts are the "nervous system" that orchestrates the various components of the `libminizinc` project, enabling the ambitious theoretical concepts to be translated into practical, self-evolving computational systems.
