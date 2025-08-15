#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <file>"
  exit 1
fi

FILE="$1"
ABS_PATH_PREFIX="/data/data/com.termux/files/home/storage/github/libminizinc/"

# Use sed to replace the absolute path prefix with ./ 
# The 's' command is for substitute.
# The '#' is used as a delimiter instead of '/' to avoid issues with '/' in the path.
# 'g' flag for global replacement (all occurrences in a line).
sed -i "s#${ABS_PATH_PREFIX}#./#g" "$FILE"

echo "Replaced absolute path prefix in $FILE"