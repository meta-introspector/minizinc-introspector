#!/bin/bash

# Script: check_dataset_equivalence.sh
# Description: Checks the equivalence of 'old' and 'new' datasets.
# Related CRQ: Show that there is an equivalence between the two datasets produced.
# Adherence: ISO 9000 (verification), Six Sigma (quality control).

set -euo pipefail
set -x

OLD_DATASET_DIR="datasets/old_dataset"
NEW_DATASET_DIR="datasets/new_dataset"

echo "\n--- Checking Dataset Equivalence ---"

# Function to get SHA256 hash of all .dzn and .parquet files in a directory
get_dataset_hash() {
    local dir=$1
    find "${dir}" -type f \( -name "*.dzn" -o -name "*.parquet" \) -print0 | sort -z | xargs -0 sha256sum | sha256sum | awk '{print $1}'
}

# 1. Compare Schemas
echo "\n--- Comparing Schemas ---"

OLD_SCHEMA_FILE="${OLD_DATASET_DIR}/schema_old.txt"
NEW_SCHEMA_FILE="${NEW_DATASET_DIR}/schema_new.txt"

# Assuming there's at least one .dzn file in each dataset for schema inspection
OLD_PARQUET_FILE=$(find "${OLD_DATASET_DIR}" -name "*.parquet" | head -n 1)
NEW_PARQUET_FILE=$(find "${NEW_DATASET_DIR}" -name "*.parquet" | head -n 1)

if [ -z "${OLD_PARQUET_FILE}" ] || [ -z "${NEW_PARQUET_FILE}" ]; then
    echo "Error: Could not find .parquet files in both datasets for schema comparison."
    exit 1
fi

cargo run --package doc_to_minizinc_data -- inspect-parquet-schema --file-path "${OLD_PARQUET_FILE}" > "${OLD_SCHEMA_FILE}"
cargo run --package doc_to_minizinc_data -- inspect-parquet-schema --file-path "${NEW_PARQUET_FILE}" > "${NEW_SCHEMA_FILE}"

diff -q "${OLD_SCHEMA_FILE}" "${NEW_SCHEMA_FILE}" > /dev/null
if [ $? -eq 0 ]; then
    echo "✅ Schemas are identical."
else
    echo "❌ Schemas are different. See ${OLD_SCHEMA_FILE} and ${NEW_SCHEMA_FILE} for details."
    diff "${OLD_SCHEMA_FILE}" "${NEW_SCHEMA_FILE}"
fi

# 2. Compare Content Hashes
echo "\n--- Comparing Content Hashes ---"

OLD_HASH=$(get_dataset_hash "${OLD_DATASET_DIR}")
NEW_HASH=$(get_dataset_hash "${NEW_DATASET_DIR}")

echo "Old Dataset Hash: ${OLD_HASH}"
echo "New Dataset Hash: ${NEW_HASH}"

if [ "${OLD_HASH}" == "${NEW_HASH}" ]; then
    echo "✅ Dataset content is identical."
else
    echo "❌ Dataset content is different."
fi

echo "\nEquivalence check complete."
