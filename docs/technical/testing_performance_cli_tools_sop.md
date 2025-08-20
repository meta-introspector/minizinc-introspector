# Standard Operating Procedure (SOP): Testing and Performance Guidelines for CLI Tools

## 1. Purpose

This SOP outlines the procedures for testing and evaluating the performance of the Command Line Interface (CLI) tools within the `libminizinc` project. It aims to ensure the reliability, correctness, and efficiency of these tools, providing guidelines for both functional and performance validation.

## 2. Scope

This SOP applies to all Rust-based CLI binaries developed as part of the `libminizinc` project, including `doc_to_minizinc_data` and `zos-bootstrap-main`, and any future CLI tools.

## 3. Functional Testing

Functional testing ensures that the CLI tools perform their intended operations correctly and produce accurate outputs.

### 3.1. General Testing Principles

*   **Input Validation:** Test with valid, invalid, and edge-case inputs to ensure robust error handling.
*   **Output Verification:** Verify that the tool's output (stdout, stderr, generated files) matches the expected results.
*   **Idempotency:** For operations that modify state (e.g., indexing), ensure that running the command multiple times produces the same result.

### 3.2. Testing `doc_to_minizinc_data`

This tool extracts words, generates embeddings, and prepares data for MiniZinc models.

*   **Basic Usage:**
    ```bash
    cargo run --package doc_to_minizinc_data -- --input-path <path_to_sample_dir_or_file>
    ```
    *   Verify that `word_embeddings_chunk_X.dzn` files are generated in `minizinc_data/huggingface/`.
    *   Inspect the contents of the generated `.dzn` files to ensure correct word mapping and embedding format.
*   **Chunking Verification:**
    ```bash
    cargo run --package doc_to_minizinc_data -- --input-path <path_to_large_sample_dir> --chunk-size 50
    ```
    *   Confirm that multiple `.dzn` files are created, each containing approximately `chunk_size` words.
*   **Input File Types:** Test with directories containing a mix of `.md`, `.rs`, `.cpp`, `.h`, and `.hpp` files to ensure all supported types are processed.
*   **Error Handling:** Test with non-existent input paths or invalid arguments.

### 3.3. Testing `zos-bootstrap-main`

This is the primary CLI for project management and ZOS bootstrapping. Test each subcommand individually.

*   **General Subcommand Testing:**
    ```bash
    cargo run --package zos-bootstrap-main -- <SUBCOMMAND> --help
    ```
    *   Verify that the help message for each subcommand is accurate and informative.
*   **`build` Subcommand:**
    ```bash
    cargo run --package zos-bootstrap-main -- build --help
    cargo run --package zos-bootstrap-main -- build --command all # Or specific build commands
    ```
    *   Verify successful compilation and output in `target/`.
*   **`test` Subcommand:**
    ```bash
    cargo run --package zos-bootstrap-main -- test --help
    cargo run --package zos-bootstrap-main -- test --command all # Or specific test commands
    ```
    *   Verify that tests run and report correct results.
*   **`bootstrap` Subcommand:**
    ```bash
    cargo run --package zos-bootstrap-main -- bootstrap --target zos
    ```
    *   Monitor the output to ensure all bootstrap steps (build, test, initial embedding) are executed.
    *   Verify the final "ZOS Bootstrap Complete" message.
*   **Code Analysis Subcommands (`extract-constants`, `analyze-constants`, `code-search`, `ast-to-minizinc`, `test-ast-to-minizinc`):**
    *   Run these commands on a sample codebase.
    *   Verify the correctness of extracted data, analysis reports, and transformations.
    *   Refer to `docs/sops/zos_codebase_querying_sop.md` for detailed usage of related analysis tools.

## 4. Performance Testing

Performance testing evaluates the speed, resource consumption (CPU, memory), and scalability of the CLI tools.

### 4.1. Measuring Execution Time

Use the `time` command (available in most Unix-like environments) to measure the wall-clock time, user CPU time, and system CPU time.

```bash
time cargo run --package <package_name> -- <args>
```

**Example for `doc_to_minizinc_data`:**
```bash
time cargo run --package doc_to_minizinc_data -- --input-path /path/to/large/codebase --chunk-size 1000
```

### 4.2. Memory Usage

*   **Linux/Android (Termux):** Use `top`, `htop`, or `procrank` (on Android) to monitor memory usage during execution.
*   **Custom Profiler:** The project uses its own "poor man's profiler" for memory and performance analysis on Android. Refer to `docs/sops/advanced_testing_profiling_ffi_v2.md` for details on setting up and using coverage and profiling tools.

### 4.3. Performance Tuning Parameters

*   **`doc_to_minizinc_data`:**
    *   `--chunk-size`: Adjusting this parameter can impact memory usage and the number of generated `.dzn` files. Smaller chunks might reduce peak memory but increase file I/O.
*   **`zos-fast-query` (underlying `zos-bootstrap-main`'s indexing):**
    *   `MAX_TOTAL_TERMS` (in `term_loader.rs`): Limits the total number of terms processed.
    *   `MAX_TERMS_PER_CHUNK` (in `chunk_generator.rs`): Controls the size of generated JSON chunks, directly impacting runtime memory.
    *   `MAX_TERMS_PER_FILE` (in `vocabulary_dfa_generator/src/main.rs`): Controls the size of generated DFA `.rs` files.

## 5. Incremental Indexing and Status Reporting

The project aims for efficient, incremental updates to its various indices.

### 5.1. Current Incremental Capabilities

*   **`file_content_analyzer` (`zos-bootstrap-main`'s underlying analysis):** The `full_analysis` mode is designed to be resumable and only re-processes changed or new files, leveraging caching.
*   **Term Generation (`zos-fast-query`):** Rebuilding `zos-fast-query` only regenerates term data if `hierarchical_term_index.json` has changed.

### 5.2. Proposed Incremental Update and Status Report Tool

To provide a more explicit and user-friendly incremental indexing experience with detailed status reports, a new subcommand will be added to `zos-bootstrap-main`, for example: `zos-bootstrap-main index-update --incremental`.

This subcommand will:
*   **Detect Changes:** Intelligently identify which source files have been modified since the last indexing run.
*   **Incremental Processing:** Only process the changed files and update the relevant parts of the index.
*   **Detailed Status Reports:** Provide real-time feedback on:
    *   Number of files scanned.
    *   Number of changed files detected.
    *   Current file being processed.
    *   Progress percentage.
    *   Estimated time remaining.
    *   Total time elapsed.
    *   Memory usage during indexing.
*   **Consolidated Index Update:** Orchestrate the updates for both word embeddings (via `doc_to_minizinc_data`'s logic) and term recognition indices (via `zos-fast-query`'s and `file_content_analyzer`'s logic).

## 6. Troubleshooting

*   **Slow Indexing:**
    *   Ensure `build_config.toml` paths are correct.
    *   Adjust `MAX_TERMS_PER_CHUNK` and `MAX_TOTAL_TERMS` if memory is an issue.
    *   Consider running on a smaller subset of the codebase using `--input-path` for `doc_to_minizinc_data`.
*   **Inconsistent Index:**
    *   Ensure all relevant tools are rebuilt after code changes (`cargo build --workspace`).
    *   If issues persist, try a full clean and rebuild (`cargo clean` followed by `cargo build --workspace`).
*   **Memory Leaks/High Usage:**
    *   Utilize the project's custom profiler or system tools (`top`, `htop`) to identify memory-intensive phases.
    *   Report issues with detailed steps to reproduce.

## 7. Related SOPs

*   [Standard Operating Procedure (SOP): ZOS Codebase Querying](docs/sops/zos_codebase_querying_sop.md)
*   [Standard Operating Procedure: Term Recognition System Generation and Modification](docs/sops/term_recognition_system_generation_sop.md)
*   [MiniZinc Model Performance Analysis and Debugging Report](docs/performance_analysis_report.md)
