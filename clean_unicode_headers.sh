#!/bin/bash

TARGET_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/gecode/third-party/boost/"

echo "Cleaning .hpp files in: $TARGET_DIR"

find "$TARGET_DIR" -name "*.hpp" -print0 | while IFS= read -r -d $'\0' file; do
    echo "Processing: $file"
    # Replace common mojibake patterns for accented characters
    # Example: replacing 'Ã©' with 'é'
    sed -i 's/Ã©/é/g' "$file"
    sed -i 's/Ã¶/ö/g' "$file"
    sed -i 's/Ã /à/g' "$file"
    sed -i 's/Ã¨/è/g' "$file"
    sed -i 's/Ãª/ê/g' "$file"
    sed -i 's/Ã¯/ï/g' "$file"
    sed -i 's/Ã»/û/g' "$file"
    sed -i 's/Ã¼/ü/g' "$file"
    sed -i 's/Ã§/ç/g' "$file"
    sed -i 's/Ã€/À/g' "$file"
    sed -i 's/Ãˆ/È/g' "$file"
    sed -i 's/ÃŠ/Ê/g' "$file"
    sed -i 's/ÃŒ/Ì/g' "$file"
    sed -i 's/Ã’/Ò/g' "$file"
    sed -i 's/Ã”/Ô/g' "$file"
    sed -i 's/Ã›/Û/g' "$file"
    sed -i 's/Ãœ/Ü/g' "$file"
    sed -i 's/Ã‡/Ç/g' "$file"

    # Specific fix for the previously identified string, in case it's still there or corrupted differently
    sed -i 's/Herv Brnnimann/Hervé Brönnimann/g' "$file"

done

echo "Cleaning complete."
