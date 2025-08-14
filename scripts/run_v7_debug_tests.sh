#!/bin/bash

# Script to run a focused set of MiniZinc models using run_embedding_model_v7.sh
# and measure their execution time for debugging purposes.

RESULTS_FILE="v7_debug_test_results.txt"
> "$RESULTS_FILE" # Clear previous results

RUN_EMBEDDING_MODEL_SCRIPT="/data/data/com.termux/files/home/storage/github/libminizinc/scripts/run_embedding_model_v7.sh"

MAIN_MODEL_VERSION="v6"
KAPPA_PARAMS_VERSION="v1"
OTHER_PARAMS_VERSION="v1"
RELATIONS_VERSION="v1"

# Focused set of core_params_versions for debugging
CORE_PARAMS_VERSIONS=("v1" "nv2" "nv7" "nv10000")

# Execute the Rust test runner
"$MINIZINC_PROJECT_ROOT/tools/minizinc_test_runner_rs/target/release/minizinc_test_runner_rs"