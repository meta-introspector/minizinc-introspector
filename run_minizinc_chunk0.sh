#!/bin/bash

# SOP-MINIZINC-SOLVE-001: Running MiniZinc Solver for Word Embedding Optimization
# Purpose: Optimize word embeddings for Chunk 0 using the MiniZinc solver.
# Adherence: ISO 9000 (verification), Six Sigma (quality control).

set -euo pipefail

MINIZINC_EXECUTABLE="/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc"
MODEL_PATH="/data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn"
DATA_FILE="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/chunk0_output/word_embeddings_chunk_0.dzn"
OUTPUT_LOG="minizinc_data/chunk0_output/minizinc_output_chunk0.log"

echo "--- Starting MiniZinc optimization for Chunk 0 ---"

# Run MiniZinc with strace for auditing file operations
STRACE_WRAPPER="/data/data/com.termux/files/home/storage/github/libminizinc/strace_wrapper.sh"

# Run MiniZinc with strace for auditing file operations
"${STRACE_WRAPPER}" "${OUTPUT_LOG}.strace" \
    "${MINIZINC_EXECUTABLE}" --solver Gecode \
    "${MODEL_PATH}" \
    "${DATA_FILE}" \
    > "${OUTPUT_LOG}" 2>&1

echo "--- MiniZinc optimization for Chunk 0 complete. Output in ${OUTPUT_LOG} ---"
echo "--- Strace log in ${OUTPUT_LOG}.strace ---"
