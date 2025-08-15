# MiniZinc FFI Debugging Progress Log

## Date: August 14, 2025

## Objective:
To get the Rust FFI for MiniZinc working correctly, specifically focusing on parsing MiniZinc models without "undefined identifier" errors or SIGSEGV crashes.

## Initial State:
*   Rust FFI tests (`test_parse_and_inspect_models`, `test_parse_model_from_string`, `test_parse_data_from_string`) were failing with "undefined identifier" errors (e.g., `min`, `domain`) and SIGSEGV.
*   Discrepancy in MiniZinc versions reported by C++ build (2.9.3) and Rust FFI (2.9.4-introspector).

## Actions Taken & Observations:

### 1. Initial Test Run & Backtrace:
*   Ran `reproduce_minizinc_ffi_bug.sh` and `RUST_BACKTRACE=full reproduce_minizinc_ffi_bug.sh`.
*   **Observation:** Tests panicked due to `parse_model` returning `Err` (MiniZinc parsing errors). SIGSEGV was a consequence of these panics. Errors were "undefined identifier" for `min` and `domain` in `stdlib_internal.mzn` and `redefinitions-2.0.2.mzn`.

### 2. Standard Library Investigation:
*   Confirmed `share/minizinc/std` contains standard library files.
*   Searched for `function int: min` and `function var set of int: domain` in `share/minizinc/std`.
*   **Observation:** Neither were found. This suggested a mismatch between the MiniZinc library and its standard library files.

### 3. Refactoring Tests to Feature-Based:
*   User requested refactoring tests to be feature-based (one test per feature).
*   Created `tools/minizinc_ffi/src/feature_tests/mod.rs`, `test_empty_model.rs`, `test_basic_int_var.rs`, `test_output_statement.rs`.
*   Modified `lib.rs` to include `feature_tests` module and removed old problematic tests.
*   **Observation:** Initial compilation errors due to incorrect module declaration (`cannot declare a non-inline module inside a block`). Fixed by moving `mod feature_tests;` to top-level `lib.rs` and adjusting `use crate::*`.

### 4. `PartialEq` Derive Fix:
*   **Observation:** New compilation error: `binary operation == cannot be applied to type base_type::MiniZincBaseType`.
*   **Action:** Added `#[derive(PartialEq)]` to `MiniZincBaseType` enum in `tools/minizinc_ffi/src/base_type.rs`.
*   **Observation:** Compilation error resolved.

### 5. `minizinc_parse_model.cpp` Filename Fix:
*   **Observation:** `test_empty_model` failed with `Error: no model file given.`.
*   **Investigation:** Found `minizinc_parse_model.cpp` hardcoding the filename to `dummy_model.mzn`.
*   **Action:** Modified `minizinc_parse_model.cpp` to use the `filename` argument passed from Rust.
*   **Action:** Modified `test_empty_model.rs` to use `"% empty model"` as content.
*   **Observation:** `test_empty_model` passed.

### 6. Direct `minizinc` Executable Test:
*   **Action:** Located `build/minizinc` executable (version 2.9.3).
*   **Action:** Ran `MZN_STDLIB_DIR=/data/data/com.termux/files/home/storage/github/libminizinc/share/minizinc /data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc /data/data/com.termux/files/home/storage/github/libminizinc/temp_test_model.mzn` (with `temp_test_model.mzn` containing `var int: x; solve satisfy;`).
*   **Observation:** The `minizinc` executable ran successfully, producing a valid solution. This was a **major breakthrough**, indicating the problem was in the FFI, not the MiniZinc library itself.

### 7. Refactoring FFI to use `MznSolver`:
*   **Hypothesis:** `MznSolver` provides a more complete MiniZinc environment initialization than `Flattener` alone.
*   **Action:** Modified `minizinc_env_new.cpp` and `minizinc_env_free.cpp` to create/free `MznSolver*` instead of `Flattener*`.
*   **Action:** Updated `minizinc_ffi_declarations.h` to reflect `MznSolver*` in FFI signatures. (This required manual `write_file` due to `replace` tool issues).
*   **Action:** Updated `lib.rs` to use `MznSolver` opaque type and adjusted FFI function signatures and `MiniZincEnvironment` methods.
*   **Observation:** New compilation errors: `improper_ctypes` warnings (fixed by adding dummy field to `MznSolver` struct) and `no method named get_version_string` (due to methods being removed during refactoring).

### 8. Current Issue: `libminizinc_c_wrapper.so` not found:
*   **Observation:** `CANNOT LINK EXECUTABLE ... library "libminizinc_c_wrapper.so" not found`. This occurs when running `cargo test` directly.
*   **Reason:** `LD_LIBRARY_PATH` is not set for `cargo test`.

## Next Steps:
1.  **Set `LD_LIBRARY_PATH` for `cargo test`:** Run `LD_LIBRARY_PATH=/data/data/com.termux/files/home/storage/github/libminizinc/build/:/data/data/com.termux/files/home/storage/github/libminizinc/install/lib/ RUST_BACKTRACE=full cargo test --package minizinc_ffi`.
2.  **Re-add `get_version_string`, `get_mznlib_dir`, `get_executable_path` methods to `impl MiniZincEnvironment`:** After the `LD_LIBRARY_PATH` issue is resolved, I will re-insert these methods into `lib.rs`.
3.  **Refactor `minizinc_parse_model.cpp` to use `MznSolver::run()`:** This is still pending and crucial for proper MiniZinc environment setup.
