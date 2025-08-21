#!/bin/bash

# SOP-DATA-GEN-002: Generating Subsequent Word Embedding Data with Fixed Embeddings
# Purpose: Generate a subsequent chunk (Chunk 1) of word embeddings, fixing embeddings from Chunk 0.
# Adherence: ISO 9000 (traceability), GMP (controlled process).

set -euo pipefail

OUTPUT_DIR="minizinc_data/chunk1_output"
CHUNK_SIZE=10
INPUT_PATH="."
PREVIOUS_EMBEDDINGS_PATH="minizinc_data/chunk0_output/word_embeddings.parquet"

echo "--- Starting data generation for Chunk 1 with fixed embeddings ---"

# Create output directory if it doesn't exist
mkdir -p "${OUTPUT_DIR}"

# Run doc_to_minizinc_data with strace for auditing file operations
strace -o "${OUTPUT_DIR}/strace_output.log" \
    cargo run --package doc_to_minizinc_data -- \
    generate-data \
    --chunk-size "${CHUNK_SIZE}" \
    --input-path "${INPUT_PATH}" \
    --output-path "${OUTPUT_DIR}" \
    --previous-embeddings-path "${PREVIOUS_EMBEDDINGS_PATH}"

echo "--- Data generation for Chunk 1 complete. Output in ${OUTPUT_DIR} ---"
echo "--- Strace log in ${OUTPUT_DIR}/strace_output.log ---"
