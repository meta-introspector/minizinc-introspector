#!/bin/bash

# This script runs the Rust FFI tests and ensures that raw LLVM profile data
# is generated for C++ code coverage.

set -euo pipefail

PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"
BUILD_COVERAGE_DIR="${PROJECT_ROOT}/build_coverage"

echo "Running Rust FFI tests to generate C++ coverage data..."

# Set LD_LIBRARY_PATH to ensure the Rust tests can find the C++ shared library
export LD_LIBRARY_PATH="${BUILD_COVERAGE_DIR}:${LD_LIBRARY_PATH:-}"

# Run cargo test. The instrumented C++ code will generate default.profraw
cargo test

echo "Rust FFI tests completed. Raw profile data (default.profraw) should be generated."
