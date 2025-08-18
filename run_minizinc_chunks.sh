#!/bin/bash

MINIZINC_EXE="/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc"
MODEL_FILE="/data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn"
DATA_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/word_embeddings_chunks"

# Ensure the output directory for logical relationships exists
LOGICAL_RELATIONSHIPS_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data"
mkdir -p "${LOGICAL_RELATIONSHIPS_DIR}"

# Generate logical relationships (if not already generated)
# This is a placeholder for now, as the Rust tool already generates it.
# In a real scenario, you might run the Rust tool here if it's not run separately.

# Iterate through chunked data files
for dzn_file in "${DATA_DIR}"/word_embeddings_chunk_*.dzn; do
    if [ -f "$dzn_file" ]; then
        echo "\n--- Running MiniZinc for chunk: $(basename "$dzn_file") ---"
        start_time=$(date +%s.%N)
        "${MINIZINC_EXE}" "${MODEL_FILE}" "${dzn_file}" "${LOGICAL_RELATIONSHIPS_DIR}/logical_relationships.dzn"
        end_time=$(date +%s.%N)
        duration=$(echo "$end_time - $start_time" | bc)
        echo "Time taken by MiniZinc for $(basename "$dzn_file"): ${duration}s"
    fi
done
