#!/bin/bash

# Comprehensive script to generate MiniZinc data and run the main model.
# Usage: $0 <num_vec> <base_size>

# Source the environment variables
source "$(dirname "$0")"/../.env

# Check if all required parameters are provided
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 <num_vec> <base_size>"
    exit 1
fi

NUM_VEC=$1
BASE_SIZE=$2

# --- Generate example_vector_params.dzn ---
VECTOR_PARAMS_DZN_FILE="${MINIZINC_DATA_DIR}/example_vector_params_nv${NUM_VEC}_bs${BASE_SIZE}.dzn"
echo "Generating ${VECTOR_PARAMS_DZN_FILE}"
${LIBMINIZINC_BUILD_DIR}/minizinc "${MINIZINC_MODELS_DIR}/generate_vector_params_v2.mzn" -D "num_vec=${NUM_VEC}; base_size=${BASE_SIZE};" -o "${VECTOR_PARAMS_DZN_FILE}"

if [ $? -ne 0 ]; then
    echo "Error: Failed to generate vector params DZN file."
    exit 1
fi
echo "Generated ${VECTOR_PARAMS_DZN_FILE}"

# --- Create example_core_params.dzn ---
CORE_PARAMS_DZN_FILE="${MINIZINC_DATA_DIR}/example_core_params_nv${NUM_VEC}.dzn"
echo "Creating ${CORE_PARAMS_DZN_FILE}"
cat <<EOF > "${CORE_PARAMS_DZN_FILE}"
% File: $(basename "${CORE_PARAMS_DZN_FILE}")
% Purpose: Defines core parameters for testing MiniZinc model runtime.
% This version sets num_vec to ${NUM_VEC}.

num_vec = ${NUM_VEC};
EOF

if [ $? -ne 0 ]; then
    echo "Error: Failed to create core params DZN file."
    exit 1
fi
echo "Created ${CORE_PARAMS_DZN_FILE}"

# --- Run the main MiniZinc model ---
MAIN_MODEL_VERSION="test_master" # Our fully included model
KAPPA_PARAMS_VERSION="dummy_v1"
OTHER_PARAMS_VERSION="dummy_v1"
RELATIONS_VERSION="test_v6" # Our latest relations with beta reduction

echo "Running main MiniZinc model with num_vec=${NUM_VEC}, base_size=${BASE_SIZE}..."
MINIZINC_COMMAND="${LIBMINIZINC_BUILD_DIR}/minizinc -v -s --time-limit 60000 --json-stream"
MINIZINC_COMMAND+=" ${MINIZINC_MODELS_DIR}/embedding_sphere_${MAIN_MODEL_VERSION}.mzn"
MINIZINC_COMMAND+=" ${CORE_PARAMS_DZN_FILE}"
MINIZINC_COMMAND+=" ${MINIZINC_DATA_DIR}/example_kappa_params_${KAPPA_PARAMS_VERSION}.dzn"
MINIZINC_COMMAND+=" ${MINIZINC_DATA_DIR}/example_other_params_${OTHER_PARAMS_VERSION}.dzn"
MINIZINC_COMMAND+=" ${MINIZINC_DATA_DIR}/example_relations_${RELATIONS_VERSION}.dzn"
MINIZINC_COMMAND+=" ${VECTOR_PARAMS_DZN_FILE}"

# Execute the command and capture output to a temporary file for parsing
TEMP_OUTPUT_FILE=$(mktemp)
eval "$MINIZINC_COMMAND" > "$TEMP_OUTPUT_FILE" 2>&1

MINIZINC_EXIT_CODE=$?

# Extract overall time from MiniZinc output
OVERALL_TIME=$(grep "Done (overall time" "$TEMP_OUTPUT_FILE" | sed -n 's/.*overall time \([0-9.]*\) s\./\1/p')

if [ $MINIZINC_EXIT_CODE -ne 0 ]; then
    echo "MiniZinc model run failed! Check $TEMP_OUTPUT_FILE for details."
    cat "$TEMP_OUTPUT_FILE" # Output full error log for debugging
    rm "$TEMP_OUTPUT_FILE"
    exit 1
else
    echo "MiniZinc model run completed. Overall time: ${OVERALL_TIME} s"
fi

rm "$TEMP_OUTPUT_FILE"