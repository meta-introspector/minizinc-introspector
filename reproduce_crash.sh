#!/bin/bash

# Script to reproduce the MiniZinc FFI crash.

# Set LD_LIBRARY_PATH to ensure the Rust executable and C wrapper can find the shared libraries.
# The paths are relative to the project root.
export LD_LIBRARY_PATH="/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper/:/data/data/com.termux/files/home/storage/github/libminizinc/install/lib/"

echo "Attempting to reproduce MiniZinc FFI crash..."
echo "LD_LIBRARY_PATH set to: $LD_LIBRARY_PATH"
echo "Running cargo test --package minizinc_ffi..."

# Run the Rust FFI tests and capture output
cargo test --package minizinc_ffi > crash_reproduce_stdout.log 2> crash_reproduce_stderr.log

# Check the exit code of the cargo test command
if [ $? -eq 0 ]; then
    echo "Tests passed. Crash not reproduced."
else
    echo "Tests failed. Crash likely reproduced."
fi

echo "Stdout logged to: crash_reproduce_stdout.log"
echo "Stderr logged to: crash_reproduce_stderr.log"
echo "Please check crash_reproduce_stderr.log for SIGSEGV or other errors."

cat crash_reproduce_stderr.log
