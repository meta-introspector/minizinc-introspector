# N00b Guide: Debugging C++ FFI with Code Coverage

## Introduction

This guide extends the existing coverage generation workflow to help debug the C++ Foreign Function Interface (FFI) by analyzing code coverage reports. It will walk you through confirming raw profile data generation, merging it, generating reports, and interpreting them to identify areas for further testing and debugging.

## Prerequisites

Ensure you have followed the steps in:
*   "N00b Guide: Reproducing the Current `libminizinc` FFI Test State" (`docs/n00b_guide_current_state.md`)
*   "N00b Guide: Generating and Analyzing C++ Code Coverage Reports" (`docs/n00b_guide_coverage_generation.md`)

## 1. Confirming Raw Profile Data (`.profraw`) Generation

Sometimes, `.profraw` files might be hidden due to `.gitignore` rules or other factors. To confirm that raw profile data is being generated after running your Rust FFI tests, you can explicitly list the contents of the `tools/minizinc_ffi/` directory, ignoring `.gitignore` rules:

```bash
ls -la tools/minizinc_ffi/
# Alternatively, using the Gemini CLI's list_directory tool:
# list_directory(path="tools/minizinc_ffi/", file_filtering_options={"respect_git_ignore": False})
```

You should see files named `test-*.profraw` (e.g., `test-12345.profraw`) and potentially `default.profraw`. The presence of these files indicates that the C++ code coverage instrumentation is working and data is being collected.

## 2. Merging Raw Profile Data

Once you've confirmed the `.profraw` files exist, you need to merge them into a single `.profdata` file. This is a necessary step before generating comprehensive coverage reports.

```bash
llvm-profdata merge -sparse ./tools/minizinc_ffi/test-*.profraw -o merged.profdata
```

This command will create a `merged.profdata` file in the root of your `libminizinc` directory.

## 3. Generating HTML and Text Coverage Reports

With the `merged.profdata` file, you can now generate detailed HTML reports and a concise text summary.

### HTML Report

The HTML report provides a visual, line-by-line breakdown of code coverage, highlighting executed and unexecuted lines.

```bash
llvm-cov show -format=html -output-dir=html_llvm_coverage_report_per_test -ignore-filename-regex='.*(test|thirdparty).*' /data/data/com.termux/files/home/storage/github/libminizinc/build_coverage/libminizinc_c_wrapper.so -instr-profile=merged.profdata
```

After execution, open `html_llvm_coverage_report_per_test/index.html` in your web browser to view the report.

### Text Summary

The text summary provides a quick overview of coverage percentages for regions, functions, lines, and branches.

```bash
llvm-cov report -ignore-filename-regex='.*(test|thirdparty).*' /data/data/com.termux/files/home/storage/github/libminizinc/build_coverage/libminizinc_c_wrapper.so -instr-profile=merged.profdata
```

This command will print the summary directly to your console.

## 4. Interpreting Coverage Reports for Debugging

Low coverage percentages indicate areas of the C++ codebase that are not being adequately tested by the Rust FFI tests. These are prime candidates for containing the memory corruption or other bugs.

*   **Focus on Low Coverage Files:** In the HTML report, look for files with low overall coverage. These files contain code paths that are not being exercised.
*   **Identify Uncovered Functions/Lines:** Within these files, identify specific functions or lines that are not covered (often highlighted in red in the HTML report).
*   **Prioritize FFI-related Code:** Pay special attention to files and functions directly involved in the FFI (e.g., `minizinc_c_wrapper_refactored/` files, `parser.cpp`, `model.cpp`, `gc.cpp`). The `parser.hh` file showed some coverage, so `lib/parser.cpp` is a good starting point for deeper investigation.

## 5. Next Steps: Devising New Tests and Fuzzing

Once you have identified uncovered or poorly covered areas, the next step is to devise new Rust FFI tests or fuzzing strategies to specifically target these code paths.

*   **Consult `docs/sops/plan_fuzzing_minizinc_for_coverage.md`:** This document provides strategies for creating new tests and fuzzing opportunities.
*   **Write Granular Tests:** Create small, focused Rust tests that specifically call the C++ FFI functions in the uncovered areas. This helps isolate the problem.
*   **Use `strace` for Targeted Debugging:** If a new test causes a crash or unexpected behavior, use `strace` on that specific test execution to pinpoint the exact system call that triggers the issue.

By systematically increasing code coverage in problematic areas, you can narrow down the location of the memory corruption and ultimately fix it.
