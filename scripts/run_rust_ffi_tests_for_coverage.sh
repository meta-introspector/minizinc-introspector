#!/bin/bash

# This script runs the Rust FFI tests and ensures that raw LLVM profile data
# is generated for C++ code coverage.

set -euo pipefail

PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"
BUILD_COVERAGE_DIR="${PROJECT_ROOT}/build_coverage"

echo "Running Rust FFI tests to generate C++ coverage data..."

# Set LD_LIBRARY_PATH to ensure the Rust tests can find the C++ shared library
export LD_LIBRARY_PATH="${BUILD_COVERAGE_DIR}:${LD_LIBRARY_PATH:-}"

# Set LLVM_PROFILE_FILE to generate a unique .profraw file per process
export LLVM_PROFILE_FILE="test-%p.profraw"

# Run cargo test. The instrumented C++ code will generate .profraw files.
# --test-threads=1 ensures tests run sequentially, potentially in separate processes.
cargo test -- --test-threads=1

echo "Rust FFI tests completed. Raw profile data (test-*.profraw) should be generated."

echo "Generating Rust code coverage report with cargo tarpaulin..."
# cargo tarpaulin generates coverage reports, typically in target/tarpaulin/
# We'll generate an LCOV report for potential future parsing.
cargo tarpaulin --workspace --out Lcov --output-dir target/tarpaulin_lcov

echo "Rust code coverage report generated in target/tarpaulin_lcov/lcov.info"
echo "A separate tool will be needed to convert this LCOV report to MiniZinc .dzn format."