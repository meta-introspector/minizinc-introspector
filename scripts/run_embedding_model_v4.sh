#!/bin/bash

# Script to run a MiniZinc model with versioned modules (Version 4)

# Source the environment variables
source "$(dirname "$0")/../.env"

# Check if a version vector is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <version_vector_string>"
    echo "Example: $0 core_v2_kappa_v1_other_v1_relations_v3_vector_v3"
    exit 1
fi

VERSION_VECTOR_STRING="$1"

# Dynamically construct the model file path based on the version vector
# This assumes a naming convention like embedding_sphere_final_<version_string>.mzn
MODEL_FILE="${MINIZINC_MODELS_DIR}/embedding_sphere_final_${VERSION_VECTOR_STRING}.mzn"

# Check if the constructed model file exists
if [ ! -f "$MODEL_FILE" ]; then
    echo "Error: Model file '$MODEL_FILE' not found."
    echo "Please ensure the model file corresponding to the version vector exists."
    exit 1
fi

"$LIBMINIZINC_BUILD_DIR/minizinc" "$MODEL_FILE" \
    "${MINIZINC_DATA_DIR}/example_core_params.dzn" \
    "${MINIZINC_DATA_DIR}/example_kappa_params.dzn" \
    "${MINIZINC_DATA_DIR}/example_other_params.dzn" \
    "${MINIZINC_DATA_DIR}/example_relations.dzn" \
    "${MINIZINC_DATA_DIR}/example_vector_params.dzn"

if [ $? -ne 0 ]; then
    echo "MiniZinc model run failed!"
    exit 1
fi

echo "MiniZinc model run completed."
