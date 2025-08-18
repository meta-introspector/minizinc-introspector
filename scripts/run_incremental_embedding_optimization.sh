#!/bin/bash

# Define paths
DOC_TO_MINIZINC_DATA_BIN="/data/data/com.termux/files/home/storage/github/libminizinc/target/debug/doc_to_minizinc_data"
MINIZINC_BIN="/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc"
MINIZINC_MODEL="/data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn"
LOGICAL_RELATIONSHIPS_DZN="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/logical_relationships.dzn"
OUTPUT_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/word_embeddings_chunks"
TEMP_FIXED_DZN="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/temp_fixed_embeddings.dzn"
LOG_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/logs"

# Ensure output and log directories exist
mkdir -p "$OUTPUT_DIR"
mkdir -p "$LOG_DIR"

echo "Generating word embedding chunks..."
# Call doc_to_minizinc_data to generate chunks
"$DOC_TO_MINIZINC_DATA_BIN" --output-dir "$OUTPUT_DIR"

# Initialize fixed embeddings (empty for the first iteration)
FIXED_EMBEDDINGS_DZN_PARAM=""

# Iterate through generated chunks and run MiniZinc
for chunk_file in "$OUTPUT_DIR"/word_embeddings_chunk_*.dzn;
do
    if [ -f "$chunk_file" ]; then
        CHUNK_NAME=$(basename "$chunk_file" .dzn)
        CHUNK_LOG_FILE="$LOG_DIR/${CHUNK_NAME}.log"
        echo "Processing chunk: ${CHUNK_NAME}"

        # Run MiniZinc with the current chunk and fixed embeddings (if any)
        if [ -n "$FIXED_EMBEDDINGS_DZN_PARAM" ]; then
            "$MINIZINC_BIN" "$MINIZINC_MODEL" "$chunk_file" "$FIXED_EMBEDDINGS_DZN_PARAM" "$LOGICAL_RELATIONSHIPS_DZN" > "$CHUNK_LOG_FILE"
        else
            "$MINIZINC_BIN" "$MINIZINC_MODEL" "$chunk_file" "$LOGICAL_RELATIONSHIPS_DZN" > "$CHUNK_LOG_FILE"
        fi

        # Extract loss from the log file
        LOSS=$(grep "Loss = " "$CHUNK_LOG_FILE" | awk '{print $3}')
        echo "  Loss for ${CHUNK_NAME}: ${LOSS}"

        # Extract optimized embeddings from MiniZinc output and prepare for next iteration
        grep -E "^(fixed_num_words|fixed_word_map|fixed_embeddings) =" "$CHUNK_LOG_FILE" > "$TEMP_FIXED_DZN"
        
        FIXED_EMBEDDINGS_DZN_PARAM="$TEMP_FIXED_DZN"

    fi
done

# Clean up temporary fixed embeddings file
rm -f "$TEMP_FIXED_DZN"

echo "Incremental embedding optimization complete."
