#!/bin/bash

# This script demonstrates how to profile and incrementally reindex
# the codebase using the file_content_analyzer, adhering to the
# "monotonic, additive" philosophy by avoiding direct code modifications.

# Ensure the file_content_analyzer is built
# cargo build

# Path to the file_content_analyzer executable
EXECUTABLE="./target/debug/file_content_analyzer"

# Root directory for indexing (adjust if needed)
INDEX_ROOT="/data/data/com.termux/files/home/storage/github/"

echo "Starting reindexing optimization demonstration..."
echo "--------------------------------------------------"

# --- Full Reindex with Timing ---
echo "\n--- 1. Full Reindex with Timing (Initial Baseline) ---"
echo "This will reindex the entire codebase. Subsequent runs will be faster due to caching."

# Use the 'time' command to measure execution duration
/usr/bin/time -v ${EXECUTABLE} --mode build_hierarchical_index --search-path ${INDEX_ROOT}

echo "\n--- 2. Incremental Reindexing (Leveraging Caching) ---"
echo "Subsequent runs of the full reindex will be faster if no files have changed,"
echo "due to the built-in caching mechanism in build_directory_index."

/usr/bin/time -v ${EXECUTABLE} --mode build_hierarchical_index --search-path ${INDEX_ROOT}

# --- Incremental Reindexing by Targeting Subdirectories ---
echo "\n--- 3. Incremental Reindexing by Targeting Subdirectories (Simulating 'N' steps) ---"
echo "You can simulate incremental processing or 'N' steps by targeting specific subdirectories."
echo "This is useful for reindexing only parts of the codebase that have changed or for parallel processing."

# Example: Reindex only the 'crates/zos-fast-query' directory
TARGET_DIR_1="${INDEX_ROOT}libminizinc/crates/zos-fast-query"
echo "\nReindexing: ${TARGET_DIR_1}"
/usr/bin/time -v ${EXECUTABLE} --mode build_hierarchical_index --search-path ${TARGET_DIR_1}

# Example: Reindex only the 'docs/technical' directory
TARGET_DIR_2="${INDEX_ROOT}libminizinc/docs/technical"
echo "\nReindexing: ${TARGET_DIR_2}"
/usr/bin/time -v ${EXECUTABLE} --mode build_hierarchical_index --search-path ${TARGET_DIR_2}

echo "\n--- 4. Manual Profiling Reports (via Puffin) ---"
echo "To get detailed profiling reports, run the executable with the --profile flag."
echo "You will need a Puffin viewer (e.g., puffin_egui) to visualize the data."
echo "Example: ${EXECUTABLE} --mode build_hierarchical_index --profile"

echo "\nOptimization demonstration complete."
