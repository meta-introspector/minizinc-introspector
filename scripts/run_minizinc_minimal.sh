#!/bin/bash

# Minimal script to run a MiniZinc model and output errors directly.

# Source the environment variables
source "$(dirname "$0")"/../.env

# Check if all required parameters are provided
if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] || [ -z "$4" ] || [ -z "$5" ] || [ -z "$6" ]; then
    echo "Usage: $0 <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>"
    exit 1
fi

MAIN_MODEL_VERSION="$1"
CORE_PARAMS_VERSION="$2"
KAPPA_PARAMS_VERSION="$3"
OTHER_PARAMS_VERSION="$4"
RELATIONS_VERSION="$5"
VECTOR_PARAMS_VERSION="$6"

# Dynamically construct the model file path
MODEL_FILE="${MINIZINC_MODELS_DIR}/embedding_sphere_${MAIN_MODEL_VERSION}.mzn"

# Dynamically construct the data file paths
CORE_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_core_params_${CORE_PARAMS_VERSION}.dzn"
KAPPA_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_kappa_params_${KAPPA_PARAMS_VERSION}.dzn"
OTHER_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_other_params_${OTHER_PARAMS_VERSION}.dzn"
RELATIONS_FILE="${MINIZINC_DATA_DIR}/example_relations_${RELATIONS_VERSION}.dzn"
VECTOR_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_vector_params_${VECTOR_PARAMS_VERSION}.dzn"

# Run MiniZinc and output directly
TEMP_STDERR_FILE=$(mktemp)
${LIBMINIZINC_BUILD_DIR}/minizinc --time-limit 60000 "$MODEL_FILE" "$CORE_PARAMS_FILE" "$KAPPA_PARAMS_FILE" "$OTHER_PARAMS_FILE" "$RELATIONS_FILE" "$VECTOR_PARAMS_FILE" 2> "$TEMP_STDERR_FILE"

MINIZINC_EXIT_CODE=$?

grep -v "processing file" "$TEMP_STDERR_FILE" >&2

# Restore MINIZINC_EXIT_CODE for subsequent checks
MINIZINC_EXIT_CODE=$?