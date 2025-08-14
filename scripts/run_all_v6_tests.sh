#!/bin/bash

# Script to run various combinations of MiniZinc models and data files
# using run_embedding_model_v6.sh and measure their execution time.

RESULTS_FILE="v6_test_results.txt"
> "$RESULTS_FILE" # Clear previous results

RUN_EMBEDDING_MODEL_SCRIPT="/data/data/com.termux/files/home/storage/github/libminizinc/scripts/run_embedding_model_v6.sh"

MAIN_MODEL_VERSION="v6"
KAPPA_PARAMS_VERSION="v1"
OTHER_PARAMS_VERSION="v1"
RELATIONS_VERSION="v1"
VECTOR_PARAMS_VERSION="v1"

# Test with different core_params_version
CORE_PARAMS_VERSIONS=("v1" "nv1" "nv2" "nv3" "nv5" "nv7" "nv10" "nv11" "nv13" "nv17" "nv100" "nv1000" "nv10000")

for CORE_V in "${CORE_PARAMS_VERSIONS[@]}"; do
    echo "Running: $MAIN_MODEL_VERSION $CORE_V $KAPPA_PARAMS_VERSION $OTHER_PARAMS_VERSION $RELATIONS_VERSION $VECTOR_PARAMS_VERSION"
    START_TIME=$(date +%s.%N)
    bash "$RUN_EMBEDDING_MODEL_SCRIPT" "$MAIN_MODEL_VERSION" "$CORE_V" "$KAPPA_PARAMS_VERSION" "$OTHER_PARAMS_VERSION" "$RELATIONS_VERSION" "$VECTOR_PARAMS_VERSION"
    END_TIME=$(date +%s.%N)
    DURATION=$(echo "$END_TIME - $START_TIME" | bc)
    echo "$DURATION $MAIN_MODEL_VERSION $CORE_V $KAPPA_PARAMS_VERSION $OTHER_PARAMS_VERSION $RELATIONS_VERSION $VECTOR_PARAMS_VERSION" >> "$RESULTS_FILE"
done

echo "--- Test Results (Sorted by Time) ---"
sort -n "$RESULTS_FILE"