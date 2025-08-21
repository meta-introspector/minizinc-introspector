#!/bin/bash

# Script: generate_datasets.sh
# Description: Generates 'old' and 'new' datasets using doc_to_minizinc_data for equivalence testing.
# Related CRQ: Show that there is an equivalence between the two datasets produced.
# Adherence: ISO 9000 (reproducibility), ITIL (standardized procedure), GMP (controlled process).

set -euo pipefail

# Define output directories
OLD_DATASET_DIR="datasets/old_dataset"
NEW_DATASET_DIR="datasets/new_dataset"

# Clean up previous runs
rm -rf "${OLD_DATASET_DIR}"
rm -rf "${NEW_DATASET_DIR}"

mkdir -p "${OLD_DATASET_DIR}"
mkdir -p "${NEW_DATASET_DIR}"

echo "Generating 'old' dataset (chunk_size=100, input=.) to ${OLD_DATASET_DIR}"
cargo run --package doc_to_minizinc_data -- generate-data --chunk-size 100 --input-path . --output-path "${OLD_DATASET_DIR}"

echo "Generating 'new' dataset (chunk_size=50, input=.) to ${NEW_DATASET_DIR}"
cargo run --package doc_to_minizinc_data -- generate-data --chunk-size 50 --input-path . --output-path "${NEW_DATASET_DIR}"

echo "Dataset generation complete."
