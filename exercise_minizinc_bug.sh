#!/bin/bash

# Define paths
MINIZINC_DATA_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data"
HUGGINGFACE_DIR="${MINIZINC_DATA_DIR}/huggingface"
SIMULATED_WORDNET_FILE="${MINIZINC_DATA_DIR}/simulated_wordnet.txt"
TEST_MODEL="/data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn"
MINIZINC_EXECUTABLE="/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc"
CHUNK_DZN_FILE="${HUGGINGFACE_DIR}/word_embeddings_chunk_0.dzn"

echo "--- Starting MiniZinc Bug Exercise Script ---"

# 1. Create simulated_wordnet.txt
echo "Creating dummy ${SIMULATED_WORDNET_FILE}..."
mkdir -p "${MINIZINC_DATA_DIR}"
echo "word1,word2,0.5" > "${SIMULATED_WORDNET_FILE}"
if [ $? -ne 0 ]; then
    echo "Error: Failed to create ${SIMULATED_WORDNET_FILE}"
    exit 1
fi
echo "${SIMULATED_WORDNET_FILE} created."

# 2. Run doc_to_minizinc_data to generate .dzn files
echo "Generating word embeddings data using doc_to_minizinc_data..."
mkdir -p "${HUGGINGFACE_DIR}" # Ensure the target directory exists
cargo run --package doc_to_minizinc_data -- --chunk-size 1 --input-path /data/data/com.termux/files/home/storage/github/libminizinc/minimal_test.rs
if [ $? -ne 0 ]; then
    echo "Error: doc_to_minizinc_data failed to run."
    exit 1
fi
echo "Data generation complete. Check ${HUGGINGFACE_DIR} for .dzn files."

# 3. Generate a temporary script to call MiniZinc later
echo "Generating temporary MiniZinc execution script..."
TEMP_MINIZINC_SCRIPT="${MINIZINC_DATA_DIR}/run_minizinc_temp.sh"
echo "#!/bin/bash" > "${TEMP_MINIZINC_SCRIPT}"
echo "timeout 10s ${MINIZINC_EXECUTABLE} ${TEST_MODEL} ${CHUNK_DZN_FILE} --time-limit 10000 --verbose --verbose-compilation" >> "${TEMP_MINIZINC_SCRIPT}"
chmod +x "${TEMP_MINIZINC_SCRIPT}"
echo "Temporary MiniZinc script generated: ${TEMP_MINIZINC_SCRIPT}"
echo "To run MiniZinc, execute: ${TEMP_MINIZINC_SCRIPT}"

# The direct MiniZinc execution and error checking are now commented out
# MINIZINC_OUTPUT=$("${MINIZINC_EXECUTABLE}" "${TEST_MODEL}" "${CHUNK_DZN_FILE}" --output-limit=10 --time-limit=10s 2>&1)
# MINIZINC_EXIT_CODE=$?

# echo "--- MiniZinc Output ---"
# echo "${MINIZINC_OUTPUT}"
# echo "--- End MiniZinc Output ---"

# # 4. Check for the expected error
# EXPECTED_ERROR="Error: type error: undefined identifier 'num_words'"
# if echo "${MINIZINC_OUTPUT}" | grep -q "${EXPECTED_ERROR}"; then
#     echo "SUCCESS: Expected bug demonstrated! MiniZinc reported 'undefined identifier'."
#     exit 0
# else
#     echo "FAILURE: Expected bug NOT demonstrated. MiniZinc did not report 'undefined identifier' or reported a different error."
#     echo "MiniZinc Exit Code: ${MINIZINC_EXIT_CODE}"
#     exit 1
# fi

