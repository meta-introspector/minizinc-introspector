# Architecture: Refactoring the MiniZinc Parser (`lib/parser.cpp`)

This document details the ongoing refactoring of the `lib/parser.cpp` file within the MiniZinc project. The primary goal of this refactoring is to improve modularity, readability, and maintainability by separating concerns, particularly the core parsing logic from file I/O operations. This refactoring also aims to address issues encountered when parsing models from strings, such as the `isModelString` corruption bug.

## Motivation for Refactoring

The original `lib/parser.cpp` contained a monolithic `parse` function responsible for:
*   Handling both file-based and string-based model parsing.
*   Managing include paths and seen models.
*   Processing `ParseWorkItem` queues.
*   Performing file existence checks and reading file contents.
*   Handling various error conditions.

This mixed responsibility led to a large, complex, and difficult-to-debug function. Specifically, issues were encountered where the `isModelString` flag within the `ParseWorkItem` struct was being corrupted or misread, causing the parser to incorrectly attempt file I/O when parsing from a string.

## Refactoring Approach: Separation of Concerns

The refactoring follows the principle of separation of concerns, aiming to create smaller, single-responsibility functions. The key architectural changes include:

1.  **Dedicated Buffer Parsing Function (`MiniZinc::parse_from_buffer`)**:
    *   A new function, `MiniZinc::parse_from_buffer`, has been introduced.
    *   **Responsibility**: This function is solely responsible for parsing a MiniZinc model from an in-memory string buffer. It does *not* perform any file I/O operations.
    *   **Input**: It takes the model content as a `std::string` and a `std::string` for its name (metadata).
    *   **Location**: Implemented in `lib/parser_refactored/parse_from_buffer.cpp` and declared in `lib/parser_refactored/parse_from_buffer.h`.

2.  **Dedicated File Utility Functions**:
    *   The `get_file_contents` utility function, previously in an anonymous namespace within `lib/parser.cpp`, has been moved to the `MiniZinc` namespace.
    *   **Responsibility**: This function is responsible for reading the content of a file into a `std::string`.
    *   **Location**: Declared in `lib/parser_refactored/parser_utils.h`.

3.  **Simplified `parse` Function (`lib/parser.cpp`)**:
    *   The main `parse` function in `lib/parser.cpp` has been simplified.
    *   **Responsibility**: It now acts as a dispatcher. If a model string is provided, it delegates the parsing to `MiniZinc::parse_from_buffer`. If filenames are provided, it will (in future refactoring phases) delegate to a dedicated file loading and parsing function.
    *   The complex `while (!files.empty())` loop, which previously handled both string and file parsing, has been removed from `lib/parser.cpp` for the string parsing path.

## New Files Introduced

As part of this refactoring, the following new files have been introduced:

*   **`lib/parser_refactored/parser_utils.h`**:
    *   Contains declarations for general parser utility functions, such as `get_file_contents`.
*   **`lib/parser_refactored/parse_from_buffer.h`**:
    *   Contains the declaration for the `MiniZinc::parse_from_buffer` function.
*   **`lib/parser_refactored/parse_from_buffer.cpp`**:
    *   Contains the implementation of the `MiniZinc::parse_from_buffer` function.

## Impact on FFI and Rust Bindings

This refactoring directly impacts the Rust FFI (`minizinc_ffi` crate) and its interaction with the MiniZinc parser:

*   **New FFI Function**: A new C++ FFI function, `minizinc_parse_model_with_flags`, was introduced to explicitly pass a boolean flag indicating whether the model is from a string. This was a workaround to address the `isModelString` corruption issue.
    *   **Declaration**: `tools/minizinc_c_wrapper_refactored/minizinc_ffi_declarations_v2.h`
    *   **Implementation**: `tools/minizinc_c_wrapper_refactored/minizinc_parse_model_with_flags.cpp`
*   **Rust FFI Bindings (`ffi_bindings.rs`)**: Updated to declare `minizinc_parse_model_with_flags` and deprecate the old `minizinc_parse_model`.
*   **Rust Environment Wrapper (`environment.rs`)**: The `parse_model` method now calls `minizinc_parse_model_with_flags`, passing `true` for the `is_model_string` flag and handling empty filenames by providing a dummy name (e.g., "dummy_model.mzn").
*   **Rust Tests (`tests/mod.rs`)**: Updated to use the refactored `parse_model` method and assert against the expected dummy filename.

## Future Work

*   The file-based parsing path in `lib/parser.cpp` still needs to be fully refactored into a dedicated `load_model_from_file` function.
*   The `isModelString` corruption bug itself (the underlying memory issue in `ParseWorkItem`) is still under investigation and will be addressed in a separate fix.
