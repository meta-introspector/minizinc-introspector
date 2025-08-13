#!/bin/bash

# Script to run the embedding_sphere.mzn model

LIBMINIZINC_BUILD_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/build"
MODEL_PATH="/data/data/com.termux/files/home/storage/github/libminizinc/embedding_sphere.mzn"
DATA_PATH="/data/data/com.termux/files/home/storage/github/libminizinc/example.dzn"

"$LIBMINIZINC_BUILD_DIR/minizinc" "$MODEL_PATH" "$DATA_PATH"

if [ $? -ne 0 ]; then
    echo "MiniZinc model run failed!"
    exit 1
fi

echo "MiniZinc model run completed."
