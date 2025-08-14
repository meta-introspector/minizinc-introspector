#!/bin/bash

# Script: qa_dzn_generation_verification.sh
# Purpose: Automates the generation and verification of MiniZinc DZN data files.
# Usage: ./qa_dzn_generation_verification.sh <num_vec> <base_size>

# Source the environment variables
source "$(dirname "$0")"/../.env

# --- Input Validation ---
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 <num_vec> <base_size>"
    exit 1
fi

NUM_VEC=$1
BASE_SIZE=$2

echo "--- QA Procedure: DZN Generation and Verification ---"
echo "Parameters: num_vec=${NUM_VEC}, base_size=${BASE_SIZE}"

# --- 1. Generate the DZN File ---
OUTPUT_DZN_FILE="${MINIZINC_DATA_DIR}/example_vector_params_nv${NUM_VEC}_bs${BASE_SIZE}.dzn"
echo "Generating DZN file: ${OUTPUT_DZN_FILE}"

${LIBMINIZINC_BUILD_DIR}/minizinc \
  "${MINIZINC_MODELS_DIR}/generate_vector_params_v2.mzn" \
  -D "num_vec=${NUM_VEC};" \
  -D "base_size=${BASE_SIZE};" | head -n -1 > "${OUTPUT_DZN_FILE}"

if [ $? -ne 0 ]; then
    echo "Error: DZN file generation failed for num_vec=${NUM_VEC}, base_size=${BASE_SIZE}."
    exit 1
fi
echo "DZN file generated successfully."

# --- 2. Verify the Generated DZN File ---
echo "Verifying DZN file: ${OUTPUT_DZN_FILE}"

${LIBMINIZINC_BUILD_DIR}/minizinc \
  "${MINIZINC_MODELS_DIR}/parse_vector_params.mzn" \
  "${OUTPUT_DZN_FILE}" \
  -D "num_vec=${NUM_VEC};"

if [ $? -ne 0 ]; then
    echo "Error: DZN file verification failed for num_vec=${NUM_VEC}, base_size=${BASE_SIZE}."
    exit 1
fi
echo "DZN file verified successfully."

echo "--- QA Procedure Completed Successfully ---"
