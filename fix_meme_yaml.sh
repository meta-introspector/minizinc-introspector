#!/bin/bash

# This script is a one-time fix to convert the old meme string format
# to the new structured YAML format in poem files.
# It uses sed for direct text manipulation.

POEMS_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems"

# Loop through all markdown files in the poems directory, excluding index.md
find "$POEMS_DIR" -type f -name "*.md" ! -name "index.md" | while read -r file;
do
    echo "Processing $file..."

    # Use sed to replace the old meme format with the new structured format
    # This sed command is complex due to multi-line replacement and capturing groups.
    # It assumes the memes are listed one per line, indented under 'memes:'
    # and follow the pattern: - "description" (template)
    sed -i '/^memes:/,/^[^[:space:]]/{ 
        /^memes:/!{
            s/^[[:space:]]*-\s*\"\(.*\)\"\s*(\s*(\(.*\)\s*))$/  - description: "\1"
    template: "\2"/
        }
    }' "$file"

    # Note: The above sed command is a simplified attempt. A more robust solution
    # might require multiple sed passes or a more powerful scripting language
    # if the meme lines have more variations or are not consistently formatted.
    # For now, this targets the most common pattern observed.

done

echo "Meme YAML formatting fix attempted for all poem files."
