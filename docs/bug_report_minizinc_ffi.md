# Bug Report: SIGSEGV and `undefined identifier 'promise_commutative'` in MiniZinc FFI on Android/Termux

## Title
`SIGSEGV` and `undefined identifier 'promise_commutative'` when parsing MiniZinc models via C++ FFI on Android/Termux (MiniZinc 2.9.3)

## Environment
*   **Operating System:** Android (Termux)
*   **MiniZinc Version:** 2.9.3 (built from source)
*   **Rust Version:** `rustc 1.88.0 (6b00bc388 2025-06-23)`
*   **CMake Version:** `cmake version 4.0.3`
*   **`libminizinc` repository:** `https://github.com/your-repo/libminizinc` (assuming our fork)

## Description
This bug report details a persistent issue encountered while developing a Rust Foreign Function Interface (FFI) for the MiniZinc library. The primary goal is to enable parsing and inspection of MiniZinc models directly from Rust.

Initial development involved refactoring the C++ FFI wrapper into smaller, modular files and creating corresponding newtype wrappers in Rust. This process successfully resolved initial `SIGSEGV` crashes and linker errors, allowing the C++ and Rust components to compile and link correctly.

However, during runtime, when attempting to parse MiniZinc models, the application consistently encounters a `SIGSEGV` (signal 11) and MiniZinc parsing errors indicating an `undefined identifier` for `promise_commutative` within its own standard library files (`stdlib_logic.mzn`, `stdlib_internal.mzn`).

`strace` analysis confirms that the relevant standard library files, including `stdlib_ann.mzn` (where `promise_commutative` is declared as an annotation), are successfully opened and read by the MiniZinc library. This suggests that the issue is not with file accessibility but rather with the MiniZinc parser's internal interpretation or recognition of this annotation.

## Steps to Reproduce
To reproduce the error, follow these steps:

1.  **Clone the `libminizinc` repository:**
    ```bash
    git clone https://github.com/your-repo/libminizinc.git
    cd libminizinc
    ```
    *(Note: Replace `https://github.com/your-repo/libminizinc.git` with the actual URL of our fork if different.)*

2.  **Ensure the `share/minizinc` directory is consistent with MiniZinc 2.9.3 source.** (This is the suspected root cause, but for reproduction, assume the current state of the cloned repo).

3.  **Build the C++ FFI wrapper and run Rust tests:**
    ```bash
    ./reproduce_minizinc_ffi_bug.sh
    ```

## Expected Behavior
MiniZinc models should parse successfully without `SIGSEGV` or `undefined identifier` errors. The Rust tests should pass, demonstrating correct parsing and inspection of model elements.

## Actual Behavior
The `reproduce_minizinc_ffi_bug.sh` script will output a `SIGSEGV` (signal 11) and MiniZinc parsing errors similar to the following:

```
MiniZinc parsing error (captured): Error: type error: undefined identifier `promise_commutative', did you mean `promise_commutative'?
/data/data/com.termux/files/home/storage/github/libminizinc/share/minizinc/std/stdlib/stdlib_logic.mzn:17.62-80
MiniZinc parsing error (captured): Error: type error: undefined identifier `promise_commutative', did you mean `promise_commutative'?
/data/data/com.termux/files/home/storage/github/libminizinc/share/minizinc/std/stdlib/stdlib_opt.mzn:79.52-65

Caused by:
  process didn't exit successfully: `/data/data/com.termux/files/home/storage/github/libminizinc/target/debug/deps/minizinc_ffi-xxxxxxxxxxxx` (signal: 11, SIGSEGV: invalid memory reference)
```

## Additional Information
*   The `strace` output confirms that `stdlib_ann.mzn` (where `annotation promise_commutative;` is defined) is successfully opened and read by the MiniZinc library.
*   The issue persists even after explicitly linking shared libraries with full paths in `build.rs` and ensuring all C++ FFI files are compiled and linked into `libminizinc_c_wrapper.so`.
*   This suggests a deeper incompatibility or bug within the MiniZinc 2.9.3 parser's handling of its own standard library annotations, rather than a simple file not found error.
