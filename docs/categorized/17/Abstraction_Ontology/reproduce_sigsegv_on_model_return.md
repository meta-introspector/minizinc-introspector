# Tutorial: Reproducing SIGSEGV on MiniZinc Model Return

This tutorial outlines the steps to reproduce a `SIGSEGV: invalid memory reference` error that occurs when parsing a MiniZinc model from a string via the Rust FFI. The issue appears to be related to the return of the `MiniZinc::Model*` pointer from C++ to Rust, where the Rust side receives an invalid pointer or `nullptr` despite the C++ parsing succeeding.

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

2.  **Ensure the `minizinc_parse_string_only` FFI function is implemented and used:**
    This involves the following files and their current state (as of the last successful build):
    *   **`tools/minizinc_c_wrapper_refactored/minizinc_ffi_declarations_v2.h`**: Contains the declaration for `minizinc_parse_string_only`.
    *   **`tools/minizinc_c_wrapper_refactored/minizinc_parse_string_only.cpp`**: Implements `minizinc_parse_string_only` and includes debug prints to trace the `Model*` pointer.
    *   **`cmake/targets/minizinc_c_wrapper.cmake`**: Configured to compile `minizinc_parse_string_only.cpp`.
    *   **`tools/minizinc_ffi/src/ffi_bindings.rs`**: Declares `minizinc_parse_string_only`.
    *   **`tools/minizinc_ffi/src/environment.rs`**: Implements `parse_string` which calls `minizinc_parse_string_only`.
    *   **`tools/minizinc_ffi/src/tests/mod.rs`**: Contains `test_parse_string` which calls `env.parse_string`.
    *   **`tools/minizinc_c_wrapper_refactored/minizinc_model_free.cpp`**: Contains debug prints to trace `minizinc_model_free` calls.

3.  **Rebuild the C++ library (if any changes were made to C++ files):**
    ```bash
    cd /data/data/com.termux/files/home/storage/github/libminizinc
    cmake --build build -j$(nproc)
    ```

**Steps to Reproduce:**

1.  **Run the Rust tests:**
    ```bash
    cd /data/data/com.termux/files/home/storage/github/libminizinc
    cargo test --package minizinc_ffi
    ```

**Expected Output (indicating the bug):**

You will observe output similar to this in the `stderr` of the `cargo test` command:

```
DEBUG: minizinc_env_new - Created MznSolver at: ...
DEBUG: minizinc_env_free - Freeing MznSolver at: ...
DEBUG: minizinc_env_new - Created MznSolver at: ...
[minizinc_parse_string_only] Starting parse process.
[minizinc_parse_string_only] model_str: var int: x; solve satisfy;
[minizinc_parse_string_only] MiniZinc::Env created.
[minizinc_parse_string_only] Model parsed successfully.
[minizinc_parse_string_only] DEBUG: Model pointer after mzn_yyparse: 0x...
[minizinc_parse_string_only] DEBUG: Model filename after mzn_yyparse: <string>
[minizinc_parse_string_only] DEBUG: Model pointer is nullptr after mzn_yyparse? false
error: test failed, to rerun pass `-p minizinc_ffi --lib`

Caused by:
  process didn't exit successfully: `/data/data/com.termux/files/home/storage/github/libminizinc/target/debug/deps/minizinc_ffi-...` (signal: 11, SIGSEGV: invalid memory reference)
```

**Explanation of the Bug:**

The debug prints confirm that the MiniZinc parser successfully processes the model string and returns a valid, non-null `MiniZinc::Model*` pointer from the C++ FFI function (`minizinc_parse_string_only`). However, the Rust test panics with `assertion failed: model.is_ok()`, indicating that `env.parse_string(model_code)` is returning an `Err` (or `nullptr`).

Crucially, the debug prints from `minizinc_model_free.cpp` are *not* observed in the output. This means the `Drop` implementation for `MiniZincModel` in Rust is not being triggered, and consequently, the `MiniZinc::Model` object allocated in C++ is never freed. This leads to a memory leak, and the `SIGSEGV` occurs when the operating system attempts to reclaim this leaked memory.

The discrepancy lies in the FFI boundary: the C++ function returns a valid pointer, but Rust interprets it as invalid or `nullptr`. This suggests a potential ABI mismatch or an issue with how the raw pointer is being handled across the language boundary. The `MiniZinc::Model` object is created with `new`, so it requires a corresponding `delete` call, which is currently not happening due to the Rust `Drop` not being invoked.
