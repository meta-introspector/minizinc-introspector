#!/bin/bash

export MINIZINC_PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"

# 🧪✨ -- MiniZinc Test Suite Orchestrator -- 🎶📊
# This script orchestrates the execution of all MiniZinc model tests using a Makefile-driven approach.
# It cleans previous results, triggers the build system, and summarizes test durations.

# Source the environment variables to get project paths and build configurations.
source "${MINIZINC_PROJECT_ROOT}/.env"

# Run make in the tests/minizinc_tests directory
make -C "${MINIZINC_PROJECT_ROOT}/tests/minizinc_tests" all

# Collect and report results (assuming make has generated time.log files)

echo "\n--- All Tests Completed ---"
echo "\n--- Test Summary ---"

# Find all time.log files and report their contents
find "${MINIZINC_PROJECT_ROOT}/tests/minizinc_tests" -name "time.log" | while read TIME_FILE;
do
  MODEL_NAME=$(basename $(dirname "$TIME_FILE"))
  DURATION=$(cat "$TIME_FILE" | grep "Duration:" | awk '{print $2, $3}')
  echo "$MODEL_NAME: $DURATION"
done