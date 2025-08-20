#!/bin/bash

# This script performs an incremental update of the hierarchical term index.
# The file_content_analyzer has a built-in caching mechanism that will
# only re-process directories and files that have changed since the last index build.

# Path to the file_content_analyzer executable
EXECUTABLE="./target/debug/file_content_analyzer"

# Root directory for indexing (adjust if needed)
INDEX_ROOT="/data/data/com.termux/files/home/storage/github/"

echo "Starting incremental index update..."
echo "This will only re-process changed files and directories."

# Run the hierarchical index build mode
# The --search-path argument ensures the entire project is considered for updates.
${EXECUTABLE} --mode build_hierarchical_index --search-path ${INDEX_ROOT}

if [ $? -eq 0 ]; then
    echo "Incremental index update completed successfully."
else
    echo "Incremental index update failed. Please check the output above for errors."
fi
