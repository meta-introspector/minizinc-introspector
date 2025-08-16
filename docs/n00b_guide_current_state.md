# N00b Guide: Reproducing the Current `libminizinc` FFI Test State

## Introduction

This guide outlines the steps to reproduce the current state of the `libminizinc` Foreign Function Interface (FFI) tests. In this state, the Rust FFI tests are able to execute and pass, but the underlying C++ `test_ffi_parser` executable (which directly tests the C++ FFI functions) aborts due to a memory corruption issue. Additionally, no C++ code coverage data is currently being generated.

This document is intended for new contributors or anyone looking to understand and reproduce the current debugging context.

## Prerequisites

Before you begin, ensure you have the following tools installed and configured on your system:

*   **Rust and Cargo:** Follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).
*   **CMake (>=3.5):** Install CMake from your system's package manager or [cmake.org](https://cmake.org/download/).
*   **C++ Compiler (Clang/GCC):** A modern C++ compiler is required. Clang is recommended for LLVM coverage.
*   **LLVM Tools (`llvm-profdata`, `llvm-cov`):** These are typically installed with LLVM. Ensure they are in your system's PATH.

## Steps to Reproduce

Follow these steps from the root directory of the `libminizinc` repository:

### 1. Clone the Repository

If you haven't already, clone the `libminizinc` repository:

```bash
git clone <repository_url>
cd libminizinc
```

### 2. Build C++ with Coverage Instrumentation

This step compiles the `libminizinc` C++ library with the necessary flags to enable code coverage data collection. It will create a `build_coverage/` directory.

```bash
./build_minizinc_with_coverage.sh
```

**Expected Output:** The build should complete successfully, with messages indicating that C++ code coverage instrumentation is enabled.

### 3. Run Rust FFI Tests

This step executes the Rust tests that interact with the C++ FFI. These tests are designed to use a single, globally managed MiniZinc environment.

```bash
./scripts/run_rust_ffi_tests_for_coverage.sh
```

**Expected Output:** The Rust tests should compile and run. You will see output indicating that several tests passed successfully. You might also see debug messages from the C++ side (e.g., `DEBUG: MiniZinc::Env constructor called.`).

### 4. (Optional) Attempt to Run C++ `test_ffi_parser` Executable

This executable directly tests the C++ FFI functions. It is known to abort due to a memory corruption issue.

```bash
LD_LIBRARY_PATH=./build_coverage:$LD_LIBRARY_PATH ./build_coverage/test_ffi_parser
```

**Expected Output:** The executable will likely run some tests and then `Aborted` with an exit code of 134. You may see messages like "Pointer tag for ... was truncated." This confirms the memory corruption issue.

## Current State and Known Issues

*   **Rust FFI Tests:** The Rust tests (specifically the granular tests in `minimal_crash_tests.rs` and `test_global_env_access`) are currently passing. This indicates that these specific test sequences do not trigger the memory corruption that leads to a crash.
*   **C++ `test_ffi_parser` Executable:** This executable consistently aborts due to a memory corruption issue. This problem is pre-existing in the repository and is not something introduced by recent changes.
*   **C++ Code Coverage:** Despite building with coverage instrumentation and running tests, no C++ code coverage data is being generated (the `default.profraw` file remains 0 bytes). This is because the underlying memory corruption issue prevents the profiling runtime from properly flushing data to disk.

## Next Steps for Debugging

To move forward with generating C++ code coverage and addressing the memory corruption, the next step is to use `strace` to observe file system interactions during the Rust test execution. This will help determine if `.profraw` files are being attempted to be written and where the process might be failing.
