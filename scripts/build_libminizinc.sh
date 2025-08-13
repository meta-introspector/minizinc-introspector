#!/bin/bash

# Script to build libminizinc

# Source the environment variables
source "$(dirname "$0")/../.env"

mkdir -p "$LIBMINIZINC_BUILD_DIR"
cd "$LIBMINIZINC_BUILD_DIR" || exit 1

echo "Configuring libminizinc build..."
cmake "$MINIZINC_PROJECT_ROOT" -DGecode_ROOT="$GECODE_BUILD_DIR"

if [ $? -ne 0 ]; then
    echo "libminizinc CMake configuration failed!"
    exit 1
fi

echo "Building libminizinc..."
make

if [ $? -ne 0 ]; then
    echo "libminizinc build failed!"
    exit 1
fi

echo "libminizinc built successfully."