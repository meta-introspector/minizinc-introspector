# Tutorial: Reproducing the `isModelString` Corruption Bug

This tutorial outlines the steps to reproduce a bug where the `isModelString` flag within MiniZinc's `ParseWorkItem` struct appears to be corrupted or misread when parsing a model from a string via the Rust FFI. This leads to the MiniZinc parser attempting to open a non-existent file, resulting in a "Cannot open file ''" error.

**Environment:**
*   Android/Termux
*   `libminizinc` project (specifically the `minizinc_ffi` Rust crate)
*   C++ compiler with GCOV/LCOV support (e.g., GCC/Clang)
*   `cmake`, `make`, `cargo`

**Prerequisites:**
1.  Ensure you have a working build of `libminizinc` with C++ code coverage instrumentation enabled. If not, follow these steps:
    *   **Enable C++ Coverage Flags:**
        Modify `libminizinc/cmake/support/coverage_setup.cmake` (if it exists) or directly modify `libminizinc/CMakeLists.txt` to include `-fprofile-arcs -ftest-coverage` flags for CXX and C compilers.
        *   **Example (additive change to `CMakeLists.txt`):**
            Add the following lines to `libminizinc/CMakeLists.txt` after `include(cmake/support/compiler_setup.cmake)`:
            ```cmake
            option(BUILD_WITH_COVERAGE "Build with C++ code coverage instrumentation" ON)
            if(BUILD_WITH_COVERAGE)
              message(STATUS "Building with C++ code coverage instrumentation")
              set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fprofile-arcs -ftest-coverage")
              set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fprofile-arcs -ftest-coverage")
              set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fprofile-arcs -ftest-coverage")
              set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -fprofile-arcs -ftest-coverage")
            endif()
            ```
    *   **Rebuild MiniZinc with Coverage:**
        ```bash
        cd /data/data/com.termux/files/home/storage/github/libminizinc
        rm -rf build # (Optional, for a clean build, but remember the "never delete" memory)
        cmake -S . -B build -DBUILD_WITH_COVERAGE=ON
        cmake --build build -j$(nproc)
        ```
        *(Note: Replace `$(nproc)` with the actual number of CPU cores, e.g., `8`)*

2.  **Ensure `lib/parser.cpp` has debug prints:**
    The bug is observed through specific debug prints in `lib/parser.cpp`. Ensure the following lines (or similar instrumentation) are present in `lib/parser.cpp` (these were added during previous debugging attempts):

    *   **Around line 170 (inside `parse` function, after `files.emplace_back` in `else if (!modelString.empty())` block):**
        ```cpp
            // DEBUG: Check ParseWorkItem after emplace_back
            std::cerr << "DEBUG_PARSE: files.back().isModelString (after emplace_back): " << (files.back().isModelString ? "true" : "false") << std::endl;
        ```
    *   **Around line 200 (inside `while (!files.empty())` loop, before `string parentPath = np.dirName;`):**
        ```cpp
            // DEBUG: Check np.isModelString immediately after getting reference
            std::cerr << "DEBUG_WHILE: np.isModelString (after getting reference): " << (np.isModelString ? "true" : "false") << std::endl;
        ```
    *   **Around line 205 (inside `while (!files.empty())` loop, before `files.pop_back();`):**
        ```cpp
            // DEBUG: Check files.back().isModelString before pop_back
            std::cerr << "DEBUG_WHILE: files.back().isModelString (before pop_back): " << (files.back().isModelString ? "true" : "false") << std::endl;
        ```
    *   **Rebuild MiniZinc after adding these prints (if you added them manually):**
        ```bash
        cd /data/data/com.termux/files/home/storage/github/libminizinc
        cmake --build build -j$(nproc)
        ```

3.  **Ensure Rust FFI is configured to pass `is_model_string`:**
    The Rust FFI (`minizinc_ffi` crate) should be configured to call a C++ function that takes an `is_model_string` boolean flag. This was done by:
    *   Creating `tools/minizinc_c_wrapper_refactored/minizinc_ffi_declarations_v2.h` with `minizinc_parse_model_with_flags` signature.
    *   Creating `tools/minizinc_c_wrapper_refactored/minizinc_parse_model_with_flags.cpp` implementing this function.
    *   Modifying `cmake/targets/minizinc_c_wrapper.cmake` to compile `minizinc_parse_model_with_flags.cpp`.
    *   Modifying `tools/minizinc_ffi/src/ffi_bindings.rs` to declare `minizinc_parse_model_with_flags`.
    *   Modifying `tools/minizinc_ffi/src/environment.rs` to call `minizinc_parse_model_with_flags` and pass `true` for `is_model_string`.
    *   Modifying `tools/minizinc_ffi/src/tests/mod.rs` to use the updated `parse_model` function.

4.  **Run the Rust tests:**
    ```bash
    cd /data/data/com.termux/files/home/storage/github/libminizinc
    cargo test --package minizinc_ffi
    ```

**Expected Output (indicating the bug):**

You will observe output similar to this in the `stderr` of the `cargo test` command:

```
DEBUG_PARSE: files.back().isModelString (after emplace_back): true
DEBUG_WHILE: np.isModelString (after getting reference): false
DEBUG_WHILE: files.back().isModelString (before pop_back): false
DEBUG_WHILE: np.fileName: ""
DEBUG_WHILE: np.isModelString: false
DEBUG_WHILE: f: ""
DEBUG_WHILE: Inside !isModelString block
DEBUG_WHILE: f (before file operations): ""
DEBUG_WHILE: fullname (after path resolution): ""
DEBUG_WHILE: FileUtils::file_exists(fullname): false
DEBUG_WHILE: file.is_open(): false
[minizinc_parse_model_with_flags] MiniZinc parsing error (captured): Error: error: Cannot open file ''.
```

**Explanation of the Bug:**

The key lines are:
*   `DEBUG_PARSE: files.back().isModelString (after emplace_back): true`
*   `DEBUG_WHILE: np.isModelString (after getting reference): false`

This shows that the `isModelString` boolean member of the `ParseWorkItem` struct is correctly set to `true` when the item is added to the `files` vector. However, when the same `ParseWorkItem` is retrieved from the `files` vector (via `files.back()`) and referenced as `np` within the `while` loop, its `isModelString` member is `false`. This indicates a memory corruption or misreading of the boolean value, likely due to padding or alignment issues of the `ParseWorkItem` struct on the AArch64 Android platform. Because `isModelString` is `false`, the MiniZinc parser incorrectly enters the file-opening logic (`if (!isModelString)` block), attempts to open an empty filename, and throws the "Cannot open file ''" error.
