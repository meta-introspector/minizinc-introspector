## Review of `reproduce_minizinc_ffi_bug.sh`

*   **Purpose:** This script is designed to reproduce a specific MiniZinc FFI bug. It builds the C++ wrapper, sets necessary environment variables (`MZN_STDLIB_DIR`, `LD_LIBRARY_PATH`), and then runs the Rust FFI tests with `--nocapture` to display their output directly.
*   **Key Commands and Dependencies:**
    *   `export MZN_STDLIB_DIR`: Sets the path to the MiniZinc standard library, crucial for MiniZinc to find its built-in functions and predicates.
    *   `cmake -B build -S .` and `cmake --build build/`: Configures and builds the C++ wrapper (and `libminizinc`). The comment `DO NOT Clean and build the C++ wrapper` and `echo DO NOT ever clean rm -rf build/*` is a strong directive, aligning with the "trust the incremental build" memory.
    *   `export LD_LIBRARY_PATH`: Sets the library path for runtime linking of the C++ wrapper and `libmzn.so`.
    *   `cargo test --package minizinc_ffi -- --nocapture`: Runs the Rust FFI tests, showing all output.
*   **Relevance to FFI, MiniZinc, and the "Big Idea":**
    *   **FFI:** Directly involved in building and testing the FFI. The explicit setting of `MZN_STDLIB_DIR` and `LD_LIBRARY_PATH` indicates common FFI setup challenges.
    *   **MiniZinc:** Directly interacts with MiniZinc by building its library and setting its standard library path. The bug it aims to reproduce is likely related to MiniZinc's behavior or its interaction with the FFI.
    *   **"Big Idea":**
        *   **Reliability:** Similar to `reproduce_crash.sh`, this script is a dedicated tool for identifying and fixing FFI bugs, which are critical for the reliability of the "big idea."
        *   **Monotonic Epic Idea:** The explicit comment "DO NOT Clean and build the C++ wrapper" is a direct reinforcement of the "trust the incremental build" and "add-only, never edit" philosophies. This shows how development practices are aligned with the project's core principles.
        *   **Debugging:** This script is a key part of the "Observe" phase of the OODA loop, providing a controlled environment to reproduce and analyze bugs.

This script is a vital part of the project's quality assurance and debugging efforts, directly supporting the stability required for the "big idea."
