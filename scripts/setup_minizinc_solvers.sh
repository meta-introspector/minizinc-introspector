#!/bin/bash

# Script to set up MiniZinc solvers (e.g., gecode.msc)

# Source the environment variables
source "$(dirname "$0")/../.env"

GECODE_FZN_EXECUTABLE="${GECODE_BUILD_DIR}/bin/fzn-gecode"

mkdir -p "$MINIZINC_USER_SOLVERS_DIR"

GECODE_MSC_CONTENT='{
    "id": "org.gecode.gecode",
    "version": "6.2.0",
    "name": "Gecode",
    "executable": "'"$GECODE_FZN_EXECUTABLE"'"
}'

echo "$GECODE_MSC_CONTENT" > "$MINIZINC_USER_SOLVERS_DIR/gecode.msc"

if [ $? -ne 0 ]; then
    echo "Failed to create gecode.msc!"
    exit 1
fi

echo "gecode.msc created successfully at $MINIZINC_USER_SOLVERS_DIR/gecode.msc"