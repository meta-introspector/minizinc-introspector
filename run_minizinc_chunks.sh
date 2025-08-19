#!/bin/bash

MINIZINC_EXE="/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc"
MODEL_FILE="/data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn"
DATA_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/word_embeddings_chunks"

# Ensure the output directory for logical relationships exists
LOGICAL_RELATIONSHIPS_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data"
mkdir -p "${LOGICAL_RELATIONSHIPS_DIR}"

# Ensure the data directory for chunks exists and is empty before generating new ones
mkdir -p "${DATA_DIR}"
rm -f "${DATA_DIR}"/word_embeddings_chunk_*.dzn

echo "--- Generating word embedding chunks ---"
cargo run --package doc_to_minizinc_data -- --input-dir /data/data/com.termux/files/home/storage/github/libminizinc --output-dir "${DATA_DIR}" --chunk-size 100
echo "--- Finished generating word embedding chunks ---"

# Generate logical relationships (if not already generated)
# This is a placeholder for now, as the Rust tool already generates it.

# Initialize fixed_embeddings.dzn for the first iteration
touch "${LOGICAL_RELATIONSHIPS_DIR}/fixed_embeddings.dzn"

chunk_count=0
for dzn_file in "${DATA_DIR}"/word_embeddings_chunk_*.dzn; do
    if [ -f "$dzn_file" ]; then
        i=$(basename "$dzn_file" | sed -n 's/word_embeddings_chunk_\([0-9]*\)\.dzn/\1/p')$(basename "$dzn_file" | sed -n 's/word_embeddings_chunk_\([0-9]*\)\.dzn/\1/p')
        TEMP_MODEL_FILE="${LOGICAL_RELATIONSHIPS_DIR}/temp_model_${i}.mzn"
        echo "include \"${dzn_file}\";" > "$TEMP_MODEL_FILE"
        # Include the fixed_embeddings.dzn from the previous iteration
        echo "include \"${LOGICAL_RELATIONSHIPS_DIR}/fixed_embeddings.dzn\";" >> "$TEMP_MODEL_FILE"
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

        # Extract and log the objective value (loss)
        OBJECTIVE_VALUE=$(grep -oP '----------\K[0-9.]+' "${LOGICAL_RELATIONSHIPS_DIR}/minizinc_output_${i}.txt" | head -n 1)
        if [ -z "$OBJECTIVE_VALUE" ]; then
            OBJECTIVE_VALUE=$(grep -oP 'objective = \K[0-9.]+' "${LOGICAL_RELATIONSHIPS_DIR}/minizinc_output_${i}.txt" | head -n 1)
        fi
        echo "Loss for chunk $(basename \"$dzn_file\"): ${OBJECTIVE_VALUE}"

        # Extract optimized embeddings and save for the next iteration
        cat "${LOGICAL_RELATIONSHIPS_DIR}/minizinc_output_${i}.txt" | cargo run --package minizinc_output_parser > "${LOGICAL_RELATIONSHIPS_DIR}/fixed_embeddings.dzn"

        rm "$TEMP_MODEL_FILE" # Clean up temporary model file
        chunk_count=$((chunk_count + 1))
        if [ "$chunk_count" -ge 2 ]; then
            echo "Processed 2 chunks. Exiting."
            break
        fi
    fi
done