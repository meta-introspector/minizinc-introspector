# N00b Guide: Generating and Analyzing C++ Code Coverage Reports

## Introduction

This guide provides a step-by-step process for generating and analyzing C++ code coverage reports for the `libminizinc` project, specifically focusing on the code exercised by the Rust Foreign Function Interface (FFI) tests. Code coverage is a crucial metric that helps identify untested parts of your codebase, improving reliability and confidence in your software.

## Prerequisites

Before following this guide, ensure you have:

*   Successfully set up your development environment and built the C++ library with coverage instrumentation by following the instructions in the "N00b Guide: Reproducing the Current `libminizinc` FFI Test State" (`docs/n00b_guide_current_state.md`).
*   `llvm-profdata` and `llvm-cov` tools installed and accessible in your system's PATH. These are typically part of the LLVM toolchain.

## Steps to Generate C++ Code Coverage Reports

Follow these steps from the root directory of the `libminizinc` repository:

### 1. Build C++ with Coverage Instrumentation

This step ensures that the C++ `libminizinc` library is compiled with the necessary flags to collect code coverage data during execution. If you have already done this, you can skip this step.

```bash
./build_minizinc_with_coverage.sh
```

**Expected Output:** The build should complete successfully, with messages indicating that C++ code coverage instrumentation is enabled.

### 2. Run Rust FFI Tests to Generate Raw Profile Data

This step executes the Rust FFI tests. As these tests interact with the instrumented C++ code, raw profile data (`.profraw` files) will be generated. Each test process will generate its own unique `.profraw` file.

```bash
./scripts/run_rust_ffi_tests_for_coverage.sh
```

**Expected Output:** The Rust tests should run and pass. You will see messages indicating the tests passed. Upon completion, you should find several `test-<PID>.profraw` files in the `tools/minizinc_ffi/` directory (where `<PID>` is the process ID).

### 3. Merge Raw Profile Data

Since multiple `.profraw` files are generated (one per test process), they need to be merged into a single `.profdata` file before generating the final report.

```bash
llvm-profdata merge -sparse ./tools/minizinc_ffi/test-*.profraw -o merged.profdata
```

**Expected Output:** A `merged.profdata` file will be created in the project root directory.

### 4. Generate HTML Coverage Report

This step processes the merged profile data and generates a detailed, human-readable HTML report. This report allows you to visually inspect which lines, functions, and branches of your C++ code were executed.

```bash
llvm-cov show -format=html -output-dir=html_llvm_coverage_report_per_test -ignore-filename-regex='.*(test|thirdparty).*' /data/data/com.termux/files/home/storage/github/libminizinc/build_coverage/libminizinc_c_wrapper.so -instr-profile=merged.profdata
```

**Expected Output:** An `html_llvm_coverage_report_per_test/` directory will be created in the project root. To view the report, open `html_llvm_coverage_report_per_test/index.html` in your web browser.

### 5. Generate Text Coverage Summary

This step provides a concise text-based summary of the C++ code coverage, including overall percentages for lines, functions, and branches.

```bash
llvm-cov report -ignore-filename-regex='.*(test|thirdparty).*' /data/data/com.termux/files/home/storage/github/libminizinc/build_coverage/libminizinc_c_wrapper.so -instr-profile=merged.profdata
```

**Expected Output:** A text summary of the C++ code coverage will be printed directly to your console.

## Analyzing the Reports

Once the reports are generated, you can analyze them to identify areas of your C++ codebase that are not being exercised by your Rust FFI tests. In the HTML report, look for:

*   **Red-highlighted lines:** These indicate lines of code that were not executed.
*   **Low coverage percentages:** Files or functions with low coverage percentages are candidates for new tests.

## Next Steps for Increasing Coverage

After identifying uncovered areas, refer to the "Plan for Fuzzing MiniZinc Language Features to Increase C++ Code Coverage" (`docs/sops/plan_fuzzing_minizinc_for_coverage.md`) for strategies on how to devise new Rust FFI tests and fuzzing opportunities to increase the coverage of the MiniZinc parser and other C++ components.
