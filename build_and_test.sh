#!/bin/bash

# Build the C++ wrapper
echo "Building C++ wrapper..."
cmake --build build/

# Run Cargo tests
echo "Running Cargo tests..."
/data/data/com.termux/files/home/storage/github/libminizinc/reproduce_crash.sh
