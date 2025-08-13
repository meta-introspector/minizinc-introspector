#!/bin/bash

# Script to set up MiniZinc solvers (e.g., gecode.msc)

USER_MINIZINC_SOLVERS_DIR="/data/data/com.termux/files/home/.minizinc/solvers"
GECODE_FZN_EXECUTABLE="/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/build/bin/fzn-gecode"

mkdir -p "$USER_MINIZINC_SOLVERS_DIR"

GECODE_MSC_CONTENT='{
    "id": "org.gecode.gecode",
    "version": "6.2.0",
    "name": "Gecode",
    "executable": "'"$GECODE_FZN_EXECUTABLE"'"
}'

echo "$GECODE_MSC_CONTENT" > "$USER_MINIZINC_SOLVERS_DIR/gecode.msc"

if [ $? -ne 0 ]; then
    echo "Failed to create gecode.msc!"
    exit 1
fi

echo "gecode.msc created successfully at $USER_MINIZINC_SOLVERS_DIR/gecode.msc"
