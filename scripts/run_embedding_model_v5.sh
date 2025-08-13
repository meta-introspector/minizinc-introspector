#!/bin/bash

# Script to run a MiniZinc model with full versioned modules and data (Version 5)

# Source the environment variables
source "$(dirname "$0")/../.env"

# Check if all required version parameters are provided
if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] || [ -z "$4" ] || [ -z "$5" ] || [ -z "$6" ]; then
    echo "Usage: $0 <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>"
    echo "Example: $0 v6 v1 v1 v1 v1 v1"
    exit 1
fi

MAIN_MODEL_VERSION="$1"
CORE_PARAMS_VERSION="$2"
KAPPA_PARAMS_VERSION="$3"
OTHER_PARAMS_VERSION="$4"
RELATIONS_VERSION="$5"
VECTOR_PARAMS_VERSION="$6"

# Dynamically construct the model file path
MODEL_FILE="${MINIZINC_MODELS_DIR}/embedding_sphere_final_${MAIN_MODEL_VERSION}.mzn"

# Dynamically construct the data file paths
CORE_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_core_params_${CORE_PARAMS_VERSION}.dzn"
KAPPA_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_kappa_params_${KAPPA_PARAMS_VERSION}.dzn"
OTHER_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_other_params_${OTHER_PARAMS_VERSION}.dzn"
RELATIONS_FILE="${MINIZINC_DATA_DIR}/example_relations_${RELATIONS_VERSION}.dzn"
VECTOR_PARAMS_FILE="${MINIZINC_DATA_DIR}/example_vector_params_${VECTOR_PARAMS_VERSION}.dzn"

# Check if the constructed model file exists
if [ ! -f "$MODEL_FILE" ]; then
    echo "Error: Model file '$MODEL_FILE' not found."
    exit 1
fi

# Check if all constructed data files exist
if [ ! -f "$CORE_PARAMS_FILE" ]; then echo "Error: Data file '$CORE_PARAMS_FILE' not found."; exit 1; fi
if [ ! -f "$KAPPA_PARAMS_FILE" ]; then echo "Error: Data file '$KAPPA_PARAMS_FILE' not found."; exit 1; fi
if [ ! -f "$OTHER_PARAMS_FILE" ]; then echo "Error: Data file '$OTHER_PARAMS_FILE' not found."; exit 1; fi
if [ ! -f "$RELATIONS_FILE" ]; then echo "Error: Data file '$RELATIONS_FILE' not found."; exit 1; fi
if [ ! -f "$VECTOR_PARAMS_FILE" ]; then echo "Error: Data file '$VECTOR_PARAMS_FILE' not found."; exit 1; fi

"$LIBMINIZINC_BUILD_DIR/minizinc" "$MODEL_FILE" \
    "$CORE_PARAMS_FILE" \
    "$KAPPA_PARAMS_FILE" \
    "$OTHER_PARAMS_FILE" \
    "$RELATIONS_FILE" \
    "$VECTOR_PARAMS_FILE"

if [ $? -ne 0 ]; then
    echo "MiniZinc model run failed!"
    exit 1
fi

echo "MiniZinc model run completed."
