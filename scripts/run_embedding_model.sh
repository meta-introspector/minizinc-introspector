#!/bin/bash

# Script to run the embedding_sphere.mzn model

LIBMINIZINC_BUILD_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/build"
MODEL_PATH="/data/data/com.termux/files/home/storage/github/libminizinc/embedding_sphere_optimized.mzn"
DATA_DIR="/data/data/com.termux/files/home/storage/github/libminizinc"

"$LIBMINIZINC_BUILD_DIR/minizinc" "$MODEL_PATH" \
    "$DATA_DIR/example_core_params.dzn" \
    "$DATA_DIR/example_kappa_params.dzn" \
    "$DATA_DIR/example_other_params.dzn" \
    "$DATA_DIR/example_relations.dzn" \
    "$DATA_DIR/example_vector_params.dzn"

if [ $? -ne 0 ]; then
    echo "MiniZinc model run failed!"
    exit 1
fi

echo "MiniZinc model run completed."