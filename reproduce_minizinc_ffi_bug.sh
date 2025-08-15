#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Define the project root directory
PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"

# Navigate to the project root
cd "$PROJECT_ROOT"

# Explicitly set MZN_STDLIB_DIR to ensure MiniZinc finds its standard library
export MZN_STDLIB_DIR="$PROJECT_ROOT/share/minizinc"

echo "Building C++ wrapper..."
# Clean and build the C++ wrapper
rm -rf build/*
cmake -B build -S .
cmake --build build/

echo "Running Cargo tests..."
# Set LD_LIBRARY_PATH for runtime linking
export LD_LIBRARY_PATH="$PROJECT_ROOT/tools/minizinc_c_wrapper/:$PROJECT_ROOT/install/lib/"

# Run cargo test with --nocapture to see stdout/stderr from tests
cargo test --package minizinc_ffi -- --nocapture

echo "Tests finished."
