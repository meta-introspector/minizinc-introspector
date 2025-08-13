#!/bin/bash

# Script to build Gecode

# Source the environment variables
source "$(dirname "$0")/../.env"

mkdir -p "$GECODE_BUILD_DIR"
cd "$GECODE_BUILD_DIR" || exit 1

echo "Configuring Gecode build..."
cmake .. -DCMAKE_POLICY_VERSION_MINIMUM=3.5

if [ $? -ne 0 ]; then
    echo "Gecode CMake configuration failed!"
    exit 1
fi

echo "Building Gecode..."
make

if [ $? -ne 0 ]; then
    echo "Gecode build failed!"
    exit 1
fi

echo "Gecode built successfully."