#!/bin/bash

# Directory to clean
TARGET_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/"

# File to modify (or loop through all relevant files in a generic script)
FILE="${TARGET_DIR}hw_rounding.hpp"

# Use sed to replace the problematic string
# The 'i' flag edits the file in place.
# The 'g' flag replaces all occurrences on a line.
sed -i 's/Herv Brnnimann/Hervé Brönnimann/g' "$FILE"

echo "Cleaned: $FILE"