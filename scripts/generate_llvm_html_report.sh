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
