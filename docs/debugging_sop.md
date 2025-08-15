# Debugging MiniZinc FFI Issues on Android/Termux

## Purpose
This SOP provides a step-by-step guide for debugging issues encountered when using the MiniZinc FFI (Foreign Function Interface) from Rust on Android/Termux. It focuses on common problems like crashes (SIGSEGV, SIGABRT) and MiniZinc parsing errors.

## Scope
This SOP applies to debugging the `minizinc_ffi` Rust crate and its underlying C++ FFI wrapper within the `libminizinc` project on Android devices running Termux.

## Prerequisites
*   A working Android device with Termux installed.
*   The `libminizinc` project cloned to your device.
*   Basic familiarity with Rust, C++, and the command line.
*   `strace` installed in Termux (`pkg install strace`).
*   `gdb` or `lldb` installed in Termux (`pkg install gdb` or `pkg install lldb`).

## Procedure

### Phase 1: Initial Triage and Information Gathering

1.  **Reproduce the Issue:**
    *   Run the minimal reproduction script: `./reproduce_minizinc_ffi_bug.sh`
    *   Observe the output. Note any error messages, especially Rust panics, C++ compilation errors, or MiniZinc parsing errors.
    *   **If a crash occurs (SIGSEGV, SIGABRT):** Proceed to Phase 2.
    *   **If a MiniZinc parsing error occurs (e.g., "undefined identifier"):** Proceed to Phase 3.
    *   **If a Rust compilation/linking error occurs:** Proceed to Phase 4.

### Phase 2: Analyzing Crashes (SIGSEGV, SIGABRT)

1.  **Get a Rust Backtrace:**
    *   Run the reproduction script with `RUST_BACKTRACE=full`: `RUST_BACKTRACE=full ./reproduce_minizinc_ffi_bug.sh`
    *   Analyze the backtrace. The top of the backtrace (closest to `__rust_begin_unwind`) will show the Rust code that panicked. Look for lines originating from `minizinc_ffi/src/lib.rs`.
    *   If the crash is in C++ code, the backtrace might show `unknown` frames or point to FFI calls.
2.  **Use `strace` for System Call Tracing:**
    *   Run `strace` to log file access and process execution: `strace -e execve,openat,mkdirat -o strace.log -f ./reproduce_minizinc_ffi_bug.sh`
    *   Examine `strace.log`. Look for:
        *   Failed `openat` calls (e.g., `ENOENT` - No such file or directory) for `.so` or `.mzn` files.
        *   Unexpected file accesses or permissions issues.
3.  **Use a Debugger (GDB/LLDB):**
    *   This is the most powerful tool for C++ crashes.
    *   **Attach to the process:**
        *   Run the test in one Termux session: `cargo test --package minizinc_ffi -- --nocapture`
        *   In another Termux session, find the PID of the test executable: `pgrep -f minizinc_ffi`
        *   Attach `gdb` (or `lldb`) to the PID: `gdb -p <PID>`
    *   **Get a backtrace:** Once attached, type `bt` (for gdb) or `bt all` (for lldb) to get a full backtrace. This will show the exact C++ function call stack leading to the crash.
    *   **Inspect variables:** Use `print <variable_name>` to inspect the values of variables at the crash site.
    *   **Set breakpoints:** Use `b <function_name>` or `b <file>:<line_number>` to set breakpoints and step through the code.

### Phase 3: Analyzing MiniZinc Parsing Errors

1.  **Identify the Error Message:** Note the exact error message from MiniZinc (e.g., `undefined identifier 'promise_commutative'`).
2.  **Identify the Source File and Line:** The error message usually includes the file and line number (e.g., `stdlib_logic.mzn:14.6-32`).
3.  **Inspect the Problematic `.mzn` File:**
    *   Read the content of the file: `cat /path/to/problematic/file.mzn`
    *   Go to the specified line and examine the context.
4.  **Search for the Undefined Identifier in Standard Library:**
    *   Use `grep -r "undefined_identifier_name" share/minizinc/` to find where the identifier is defined or used in other standard library files.
5.  **Hypothesize Version Mismatch:** If the identifier is defined elsewhere, but the parser still complains, it strongly suggests a version incompatibility between the `libmzn.so` and the `.mzn` files.
6.  **Simplify the MiniZinc Model:**
    *   Create a new, minimal `.mzn` file that *only* contains the problematic construct or the smallest possible model that triggers the error. This helps isolate the issue.

### Phase 4: Analyzing Rust Compilation/Linking Errors

1.  **Read the Error Message Carefully:** Rust's compiler errors are usually very descriptive.
2.  **Check `build.rs`:** Ensure `rustc-link-search` and `rustc-link-lib` directives are correct and point to the right locations.
3.  **Verify Shared Library Contents:** Use `nm` on `build/libminizinc_c_wrapper.so` to confirm that the expected symbols are present and exported.
4.  **Check `LD_LIBRARY_PATH`:** Ensure the `LD_LIBRARY_PATH` environment variable is correctly set in your shell before running tests.

## General Debugging Tips
*   **Incremental Changes:** Make small, isolated changes and test frequently.
*   **Debug Prints:** Use `println!` in Rust and `std::cerr << "DEBUG: " << ... << std::endl; std::cerr.flush();` in C++ to trace execution flow and variable values.
*   **Read Documentation:** Consult the MiniZinc documentation (if available) for specific features or error messages.
*   **Search Online:** Use search engines and forums (MiniZinc community, Rust forums, Termux community) to look for similar issues.

## Reporting a Bug
If you identify a bug, create a detailed bug report using the `docs/bug_report_minizinc_ffi.md` template and provide a minimal reproduction script (`reproduce_minizinc_ffi_bug.sh`).
