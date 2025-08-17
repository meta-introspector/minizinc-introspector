## Review of `minimal_crash_tests.rs`

*   **Purpose:** This file appears to be a collection of minimal tests, seemingly duplicated from `test_01_init_global_env.rs` through `test_05_full_lifecycle_no_filepath.rs`. The name `minimal_crash_tests.rs` suggests its purpose is to quickly reproduce or verify fixes for crashes, possibly by running a subset of the core FFI tests.
*   **Key Functions, Structs, and FFI Interactions:**
    *   It contains `test_01_init_global_env`, `test_02_parse_minimal_model`, `test_03_unwrap_model`, `test_04_get_model_filename`, and `test_05_full_lifecycle_no_filepath`. These are identical to the individual test files reviewed previously.
    *   It uses `crate::tests::tests::GLOBAL_MINIZINC_ENV` to access the global environment.
    *   It performs `env.parse_string()`, `model.unwrap()`, and `model_obj.filename()`, all of which involve FFI interactions.
*   **Relevance to FFI, MiniZinc, and the "Big Idea":**
    *   **FFI:** Directly tests core FFI functionalities (environment initialization, parsing, model unwrapping, filename retrieval).
    *   **MiniZinc:** Tests basic MiniZinc interaction through the FFI.
    *   **"Big Idea":**
        *   **Debugging and Reliability:** The existence of a dedicated "minimal crash tests" file indicates a strong focus on quickly identifying and verifying fixes for critical FFI issues. This is paramount for the reliability of the semantic embedding pipeline.
        *   **Duplication (Potential):** This file appears to duplicate tests already present in separate files. This is an area for potential improvement in terms of code organization and adherence to the "Monotonic Epic Idea" (if these are truly duplicates and not intended for a specific, separate test run).

*   **Integration into Rust Busy Box (`ragit-bootstrap`):**
    *   **Command Mapping:** These tests would fall under `ragit-bootstrap test rust-ffi`.
    *   **Internal Module:** The tests themselves would be called by the `commands/test.rs` module. The individual functionalities tested (environment, parsing, etc.) are part of the `minizinc_ffi` crate's `environment` and `model` modules.
    *   **Improvement Idea for Busy Box:** The `ragit-bootstrap debug` command could have a sub-command like `ragit-bootstrap debug minimal-ffi-crash` that specifically runs *only* these tests, providing a quick way to check for regressions on known crash scenarios. This would make the busy box a more targeted debugging tool.

This file is important for FFI stability, but its contents suggest potential duplication that could be addressed for better code organization.
