# SOP: Workflow for Increasing C++ Code Coverage ("Oxidation")

## 1. Purpose
This Standard Operating Procedure (SOP) provides a step-by-step guide for developers to increase the C++ code coverage of the `libminizinc` library by its Rust FFI tests. This process, metaphorically termed "increasing oxidation," is crucial for enhancing the reliability and quality of the Rust-C++ integration.

## 2. Scope
This SOP applies to all development activities involving the Rust FFI for `libminizinc`, specifically when aiming to improve the test coverage of the underlying C++ codebase.

## 3. Prerequisites
*   Familiarity with the overall "Plan for Systemic Increase of Rust-MiniZinc FFI C++ Code Coverage ('Oxidation')" (`docs/sops/plan_increase_oxidation.md`).
*   A working development environment with Rust, Cargo, CMake, and LLVM tools (`llvm-profdata`, `llvm-cov`). 

## 4. Workflow Steps

Follow these steps to build, test, and generate coverage reports for the C++ `libminizinc` code exercised by Rust FFI tests.

### 4.1. Build C++ with Coverage Instrumentation

This step compiles the `libminizinc` C++ library with the necessary flags to enable code coverage data collection.

**Action:** Execute the `build_minizinc_with_coverage.sh` script from the project root.

```bash
./build_minizinc_with_coverage.sh
```

**Expected Outcome:** The `libminizinc` library will be built into the `build_coverage/` directory, ready to generate profile data upon execution.

### 4.2. Run Rust FFI Tests and Generate Raw Profile Data

This step executes the Rust tests that interact with the C++ FFI. During their execution, the instrumented C++ code will generate raw profile data (`.profraw` files).

**Action:** Execute the `run_rust_ffi_tests_for_coverage.sh` script from the project root.

```bash
./scripts/run_rust_ffi_tests_for_coverage.sh
```

**Expected Outcome:** The Rust tests will run, and a `default.profraw` file (or similar) will be generated in the project root, containing the raw coverage data.

### 4.3. Generate HTML Coverage Report

This step processes the raw profile data and generates a human-readable HTML report, allowing for detailed visual analysis of C++ code coverage.

**Action:** Execute the `generate_llvm_html_report.sh` script from the project root.

```bash
./scripts/generate_llvm_html_report.sh
```

**Expected Outcome:** An `html_llvm_coverage_report/` directory will be created in the project root, containing the HTML coverage report. Open `html_llvm_coverage_report/index.html` in a web browser to view the report.

### 4.4. Generate Text Coverage Summary

This step provides a concise text summary of the C++ code coverage, including the overall "oxidation" percentage.

**Action:** Execute the `generate_llvm_text_summary.sh` script from the project root.

```bash
./scripts/generate_llvm_text_summary.sh
```

**Expected Outcome:** A text summary of the C++ code coverage will be printed to the console, including the overall lines coverage percentage.

### 4.5. Analyze Coverage Gaps and Implement New Tests

Refer to the "Plan for Systemic Increase of Rust-MiniZinc FFI C++ Code Coverage ('Oxidation')" (`docs/sops/plan_increase_oxidation.md`) for detailed guidance on analyzing coverage reports, identifying gaps, and implementing new Rust FFI tests to increase coverage.

## 5. Scripts

The following scripts are essential for executing the workflow described above.

### 5.1. `build_minizinc_with_coverage.sh`

(This script already exists in the project root)

```bash
#!/bin/bash

# This script builds libminizinc with C++ code coverage instrumentation.
# It adheres to the "add-only, never edit" development philosophy by
# creating a new build directory and not modifying existing CMake files.

set -euo pipefail

BUILD_DIR="build_coverage"
SOURCE_DIR="/data/data/com.termux/files/home/storage/github/libminizinc"

echo "Creating build directory: ${BUILD_DIR}"
mkdir -p "${BUILD_DIR}"

echo "Configuring CMake with coverage flags..."
cmake -S "${SOURCE_DIR}" -B "${BUILD_DIR}" \
  -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_CXX_FLAGS="-fprofile-arcs -ftest-coverage" \
  -DCMAKE_C_FLAGS="-fprofile-arcs -ftest-coverage"

echo "Building libminizinc with coverage instrumentation..."
cmake --build "${BUILD_DIR}" -j$(nproc)

echo "Build complete. Coverage data will be generated when tests are run."
echo "You can find the build artifacts in: ${BUILD_DIR}"
```

### 5.2. `scripts/run_rust_ffi_tests_for_coverage.sh`

```bash
#!/bin/bash

# This script runs the Rust FFI tests and ensures that raw LLVM profile data
# is generated for C++ code coverage.

set -euo pipefail

PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"
BUILD_COVERAGE_DIR="${PROJECT_ROOT}/build_coverage"

echo "Running Rust FFI tests to generate C++ coverage data..."

# Set LD_LIBRARY_PATH to ensure the Rust tests can find the C++ shared library
export LD_LIBRARY_PATH="${BUILD_COVERAGE_DIR}:${LD_LIBRARY_PATH}"

# Run cargo test. The instrumented C++ code will generate default.profraw
cargo test

echo "Rust FFI tests completed. Raw profile data (default.profraw) should be generated."
```

### 5.3. `scripts/generate_llvm_html_report.sh`

```bash
#!/bin/bash

# This script processes the raw LLVM profile data and generates an HTML coverage report.

set -euo pipefail

PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"
BUILD_COVERAGE_DIR="${PROJECT_ROOT}/build_coverage"
LIB_C_WRAPPER_SO="${BUILD_COVERAGE_DIR}/libminizinc_c_wrapper.so"
PROFRAW_FILE="${PROJECT_ROOT}/default.profraw"
PROFDATA_FILE="${PROJECT_ROOT}/default.profdata"
HTML_REPORT_DIR="${PROJECT_ROOT}/html_llvm_coverage_report"

echo "Merging raw profile data..."
llvm-profdata merge -sparse "${PROFRAW_FILE}" -o "${PROFDATA_FILE}"

echo "Generating HTML coverage report in ${HTML_REPORT_DIR}..."
mkdir -p "${HTML_REPORT_DIR}"
llvm-cov show -format=html -output-dir="${HTML_REPORT_DIR}" \
  -ignore-filename-regex='.*(test|thirdparty).*' \
  "${LIB_C_WRAPPER_SO}" -instr-profile="${PROFDATA_FILE}"

echo "HTML coverage report generated. Open ${HTML_REPORT_DIR}/index.html in your browser."
```

### 5.4. `scripts/generate_llvm_text_summary.sh`

```bash
#!/bin/bash

# This script processes the raw LLVM profile data and generates a text summary of the coverage.

set -euo pipefail

PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"
BUILD_COVERAGE_DIR="${PROJECT_ROOT}/build_coverage"
LIB_C_WRAPPER_SO="${BUILD_COVERAGE_DIR}/libminizinc_c_wrapper.so"
PROFRAW_FILE="${PROJECT_ROOT}/default.profraw"
PROFDATA_FILE="${PROJECT_ROOT}/default.profdata"

echo "Merging raw profile data..."
llvm-profdata merge -sparse "${PROFRAW_FILE}" -o "${PROFDATA_FILE}"

echo "Generating text coverage summary..."
llvm-cov report \
  -ignore-filename-regex='.*(test|thirdparty).*' \
  "${LIB_C_WRAPPER_SO}" -instr-profile="${PROFDATA_FILE}"
```
