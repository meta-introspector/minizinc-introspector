#!/bin/bash

# This script demonstrates how to build the hierarchical term index
# using the file_content_analyzer.

echo "Building hierarchical term index..."

# Run the file_content_analyzer in build_hierarchical_index mode
# Ensure you have run 'cargo build' in the libminizinc directory first.
# Also, ensure 'full_analysis' has been run at least once to generate
# the necessary project summaries (.file_analysis_summary.json files).
cargo run --package file_content_analyzer -- --mode build_hierarchical_index

if [ $? -ne 0 ]; then
    echo "ERROR: Building hierarchical term index failed."
    exit 1
fi

echo "Hierarchical term index built successfully."
echo "The index is saved to hierarchical_term_index.json in the root of your search directory."
