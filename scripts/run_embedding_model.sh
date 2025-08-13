#!/bin/bash

# Script to run the embedding_sphere.mzn model

# Source the environment variables
source "$(dirname "$0")/../.env"

MODEL_PATH="${MINIZINC_MODELS_DIR}/embedding_sphere_final.mzn"

"$LIBMINIZINC_BUILD_DIR/minizinc" "$MODEL_PATH" \
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
