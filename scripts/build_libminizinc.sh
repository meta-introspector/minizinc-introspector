#!/bin/bash

# Script to build libminizinc

LIBMINIZINC_BUILD_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/build"
LIBMINIZINC_ROOT_DIR="/data/data/com.termux/files/home/storage/github/libminizinc"
GECODE_BUILD_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/build"

mkdir -p "$LIBMINIZINC_BUILD_DIR"
cd "$LIBMINIZINC_BUILD_DIR" || exit 1

echo "Configuring libminizinc build..."
cmake "$LIBMINIZINC_ROOT_DIR" -DGecode_ROOT="$GECODE_BUILD_DIR"

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
