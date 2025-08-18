#!/bin/bash

FILES=(
    "/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/hw_rounding.hpp"
    "/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/interval.hpp"
    "/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/rounded_arith.hpp"
    "/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/rounded_transc.hpp"
    "/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/rounding.hpp"
    "/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/transc.hpp"
    "/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/numeric/interval/utility.hpp"
)

echo "Cleaning specific .hpp files for problematic Unicode characters."

for file in "${FILES[@]}"; do
    echo "Processing: $file"
    # Replace common problematic non-ASCII characters with ASCII equivalents or remove
    # Using '|' as delimiter for sed to avoid issues with single quotes.
    sed -i 's|\xE2\x80\x9C|"|g' "$file" # Left double quote
    sed -i 's|\xE2\x80\x9D|"|g' "$file" # Right double quote
    sed -i 's|\xE2\x80\x98|\'|g' "$file" # Left single quote
    sed -i 's|\xE2\x80\x99|\'|g' "$file" # Right single quote
    sed -i 's|\xE2\x80\x93|-|g' "$file" # En dash
    sed -i 's|\xE2\x80\x94|--|g' "$file" # Em dash
    sed -i 's|\xE2\x80\xA6|...|g' "$file" # Ellipsis
    sed -i 's|\xC2\xA0| |g' "$file" # Non-breaking space

    # Specific fix for the previously identified string, in case it's still there or corrupted differently
    sed -i 's|Herv Brnnimann|Hervé Brönnimann|g' "$file"
    # Also try to fix the original mojibake if it's still present
    sed -i 's|Herv\xC3\xA9 Br\xC3\xB6nnimann|Hervé Brönnimann|g' "$file" # Common UTF-8 mojibake for é and ö

done

echo "Cleaning complete."