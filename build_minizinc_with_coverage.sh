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
  -DCMAKE_C_FLAGS="-fprofile-arcs -ftest-coverage" \
  -DBUILD_WITH_COVERAGE=ON

echo "Building libminizinc with coverage instrumentation..."
cmake --build "${BUILD_DIR}" -j15

echo "Build complete. Coverage data will be generated when tests are run."
echo "You can find the build artifacts in: ${BUILD_DIR}"
