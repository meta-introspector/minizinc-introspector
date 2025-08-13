#!/bin/bash

# Script to run a MiniZinc model (Version 3)

# Source the environment variables
source "$(dirname "$0")/../.env"

# Check if a model path is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <model_file.mzn>"
    exit 1
fi

MODEL_FILE="$1"

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
