#!/bin/bash

# strace_wrapper.sh
# Purpose: A generic wrapper to run a command with strace, filtering for open calls and following child processes.
# Usage: ./strace_wrapper.sh <output_log_path> <command> [args...]
# Adherence: ISO 9000 (traceability), GMP (controlled process).

set -euo pipefail

OUTPUT_LOG="$1"
shift

# Run the provided command with strace
strace -f -e trace=open -o "${OUTPUT_LOG}" "$@"
