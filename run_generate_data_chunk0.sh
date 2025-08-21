#!/bin/bash

# SOP-DATA-GEN-001: Generating Word Embedding Data with doc_to_minizinc_data
# Purpose: Generate initial chunk (Chunk 0) of word embeddings.
# Adherence: ISO 9000 (traceability), GMP (controlled process).

set -euo pipefail

OUTPUT_DIR="minizinc_data/chunk0_output"
CHUNK_SIZE=10
INPUT_PATH="."

echo "--- Starting data generation for Chunk 0 ---"

# Create output directory if it doesn't exist
mkdir -p "${OUTPUT_DIR}"

# Run doc_to_minizinc_data with strace for auditing file operations
STRACE_WRAPPER="/data/data/com.termux/files/home/storage/github/libminizinc/strace_wrapper.sh"

# Run doc_to_minizinc_data with strace for auditing file operations
"${STRACE_WRAPPER}" "${OUTPUT_DIR}/strace_output.log" \
    cargo run --package doc_to_minizinc_data -- \
    generate-data \
    --chunk-size "${CHUNK_SIZE}" \
    --input-path "${INPUT_PATH}" \
    --output-path "${OUTPUT_DIR}"

echo "--- Data generation for Chunk 0 complete. Output in ${OUTPUT_DIR} ---"
echo "--- Strace log in ${OUTPUT_DIR}/strace_output.log ---"
