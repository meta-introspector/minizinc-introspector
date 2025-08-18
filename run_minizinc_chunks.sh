#!/bin/bash

MINIZINC_EXE="/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc"
MODEL_FILE="/data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn"
DATA_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/word_embeddings_chunks"

# Ensure the output directory for logical relationships exists
LOGICAL_RELATIONSHIPS_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data"
mkdir -p "${LOGICAL_RELATIONSHIPS_DIR}"

# Generate logical relationships (if not already generated)
# This is a placeholder for now, as the Rust tool already generates it.

chunk_count=0
for dzn_file in "${DATA_DIR}"/word_embeddings_chunk_*.dzn; do
    if [ -f "$dzn_file" ]; then
        i=$(basename "$dzn_file" | sed -n 's/word_embeddings_chunk_\([0-9]*\)\.dzn/\1/p')$(basename "$dzn_file" | sed -n 's/word_embeddings_chunk_\([0-9]*\)\.dzn/\1/p')
        TEMP_MODEL_FILE="${LOGICAL_RELATIONSHIPS_DIR}/temp_model_${i}.mzn"
        echo "include \"${dzn_file}\";" > "$TEMP_MODEL_FILE"
        echo "include \"${LOGICAL_RELATIONSHIPS_DIR}/logical_relationships.dzn\";" >> "$TEMP_MODEL_FILE"
        echo "include \"${LOGICAL_RELATIONSHIPS_DIR}/co_occurrence_data.dzn\";" >> "$TEMP_MODEL_FILE"
        echo "include \"${MODEL_FILE}\";" >> "$TEMP_MODEL_FILE"

        echo "\n--- Running MiniZinc for chunk: $(basename \"$dzn_file\") ---"
        start_time=$(date +%s.%N)
        "$MINIZINC_EXE" "$TEMP_MODEL_FILE" > "${LOGICAL_RELATIONSHIPS_DIR}/minizinc_output_${i}.txt"
        cat "${LOGICAL_RELATIONSHIPS_DIR}/minizinc_output_${i}.txt"
        ls -l "${LOGICAL_RELATIONSHIPS_DIR}/minizinc_output_${i}.txt"
        end_time=$(date +%s.%N)
        duration=$(echo "$end_time - $start_time" | bc)
        echo "Time taken by MiniZinc for $(basename \"$dzn_file\"): ${duration}s"

        rm "$TEMP_MODEL_FILE" # Clean up temporary model file
        chunk_count=$((chunk_count + 1))
        if [ "$chunk_count" -ge 2 ]; then
            echo "Processed 2 chunks. Exiting."
            break
        fi
    fi
done