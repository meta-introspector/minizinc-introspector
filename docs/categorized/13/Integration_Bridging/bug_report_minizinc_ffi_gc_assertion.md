**Title:** MiniZinc FFI: `GC::alloc` assertion failure and `SIGABRT` during model parsing via `minizinc_ffi` Rust crate.

**Description:**
When attempting to parse MiniZinc models using the `minizinc_ffi` Rust crate, specifically through the `minizinc_parse_string_only` FFI function, the application crashes with a `SIGABRT` signal. Debug output indicates an `assertion "locked()" failed` originating from `lib/gc.cpp:420`, and sometimes an `ASTStringData::destroy()` assertion failure. This suggests a fundamental issue with MiniZinc's internal garbage collection and string interning mechanisms when invoked via the FFI.

**Steps to Reproduce:**

1.  Ensure the `libminizinc` project is cloned and set up.
2.  Navigate to the project root: `/data/data/com.termux/files/home/storage/github/libminizinc`
3.  Clean the C++ build directories (if they exist):
    `rm -rf build build_coverage`
4.  Configure and build the C++ wrapper libraries:
    `cmake -B build`
    `cmake --build build`
    `cmake -B build_coverage -DCMAKE_BUILD_TYPE=Coverage`
    `cmake --build build_coverage`
5.  Run the Rust tests for the `minizinc_ffi` crate:
    `cargo test --package minizinc_ffi`

**Expected Behavior:**
All tests in the `minizinc_ffi` crate should pass successfully, and MiniZinc models should be parsed without any assertion failures or crashes.

**Actual Behavior:**
The `cargo test --package minizinc_ffi` command fails. The output shows:
*   `process didn't exit successfully: ... (signal: 6, SIGABRT: process abort signal)`
*   `lib/gc.cpp:420: void *MiniZinc::GC::alloc(size_t): assertion "locked()" failed`
*   Occasionally, `include/minizinc/aststring.hh:193: void MiniZinc::ASTStringData::destroy() const: assertion "interner().find({this->c_str(), this->size()}) != interner().end()" failed`
*   The debug message `[minizinc_parse_string_only] MiniZinc::Env created.` (or similar from `minizinc_parse_model_with_flags.cpp` or `minizinc_parse_model_v2.cpp`) appears in the output, indicating that a local `MiniZinc::Env` object is being created.

**Environment:**
*   **Operating System:** Android (Termux)
*   **Project Root:** `/data/data/com.termux/files/home/storage/github/libminizinc`
*   **MiniZinc Version:** 2.9.3 (as reported by CMake configuration)
*   **Rust Version:** `rustc 1.88.0 (6b00bc388 2025-06-23)`
*   **CMake Version:** `cmake version 4.0.3`
*   **Compiler:** Clang 20.1.7 (as reported by CMake)

**Additional Context:**
The issue appears to stem from a conflict in `MiniZinc::Env` object management. The `minizinc_ffi` attempts to manage a single `MiniZinc::MznSolver` (which encapsulates a `MiniZinc::Env`) for the FFI session. However, during parsing operations (e.g., `minizinc_parse_string_only`), it seems a new, local `MiniZinc::Env` object is implicitly created within the MiniZinc C++ library's parsing routines. This leads to multiple `Env` instances attempting to manage the same global resources (like the garbage collector and string interner), resulting in assertion failures and crashes.

Previous attempts to explicitly pass the `MznSolver`'s `Env` to `MiniZinc::ParserState` constructors were unsuccessful due to API mismatches or the `ParserState` not being designed to accept an external `Env` in that manner.
