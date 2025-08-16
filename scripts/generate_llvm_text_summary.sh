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
