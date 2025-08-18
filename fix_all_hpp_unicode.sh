#!/bin/bash

TARGET_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/"

echo "Fixing Unicode characters in all .hpp files under: $TARGET_DIR"

find "$TARGET_DIR" -name "*.hpp" -exec sed -i 's/Herv Brnnimann/Hervé Brönnimann/g' {} \; 

echo "All .hpp files processed."