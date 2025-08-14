#!/bin/bash

# Script to run a MiniZinc model with full versioned modules and data (Version 6 - with Proof Tape)

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

# Generate a unique timestamp for the proof tape
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
PROOF_TAPE_DIR="${MINIZINC_PROJECT_ROOT}/proof_tapes/${TIMESTAMP}"
mkdir -p "$PROOF_TAPE_DIR"

echo "--- Generating Proof Tape for Run: $TIMESTAMP ---"
echo "Proof Tape Directory: $PROOF_TAPE_DIR"

# Record the version vector
echo "Main Model Version: $MAIN_MODEL_VERSION" > "${PROOF_TAPE_DIR}/version_vector.txt"
echo "Core Params Version: $CORE_PARAMS_VERSION" >> "${PROOF_TAPE_DIR}/version_vector.txt"
echo "Kappa Params Version: $KAPPA_PARAMS_VERSION" >> "${PROOF_TAPE_DIR}/version_vector.txt"
echo "Other Params Version: $OTHER_PARAMS_VERSION" >> "${PROOF_TAPE_DIR}/version_vector.txt"
echo "Relations Version: $RELATIONS_VERSION" >> "${PROOF_TAPE_DIR}/version_vector.txt"
echo "Vector Params Version: $VECTOR_PARAMS_VERSION" >> "${PROOF_TAPE_DIR}/version_vector.txt"

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
    echo "Error: Model file '$MODEL_FILE' not found." | tee -a "${PROOF_TAPE_DIR}/error.log"
    exit 1
fi

# Check if all constructed data files exist
if [ ! -f "$CORE_PARAMS_FILE" ]; then echo "Error: Data file '$CORE_PARAMS_FILE' not found." | tee -a "${PROOF_TAPE_DIR}/error.log"; exit 1; fi
if [ ! -f "$KAPPA_PARAMS_FILE" ]; then echo "Error: Data file '$KAPPA_PARAMS_FILE' not found." | tee -a "${PROOF_TAPE_DIR}/error.log"; exit 1; fi
if [ ! -f "$OTHER_PARAMS_FILE" ]; then echo "Error: Data file '$OTHER_PARAMS_FILE' not found." | tee -a "${PROOF_TAPE_DIR}/error.log"; exit 1; fi
if [ ! -f "$RELATIONS_FILE" ]; then echo "Error: Data file '$RELATIONS_FILE' not found." | tee -a "${PROOF_TAPE_DIR}/error.log"; exit 1; fi
if [ ! -f "$VECTOR_PARAMS_FILE" ]; then echo "Error: Data file '$VECTOR_PARAMS_FILE' not found." | tee -a "${PROOF_TAPE_DIR}/error.log"; exit 1; fi

# Copy model and data files to proof tape for complete reproducibility
cp "$MODEL_FILE" "$PROOF_TAPE_DIR/"
cp "$CORE_PARAMS_FILE" "$PROOF_TAPE_DIR/"
cp "$KAPPA_PARAMS_FILE" "$PROOF_TAPE_DIR/"
cp "$OTHER_PARAMS_FILE" "$PROOF_TAPE_DIR/"
cp "$RELATIONS_FILE" "$PROOF_TAPE_DIR/"
cp "$VECTOR_PARAMS_FILE" "$PROOF_TAPE_DIR/"

# Run MiniZinc and capture output
echo "Current working directory: $(pwd)"
MINIZINC_COMMAND="$LIBMINIZINC_BUILD_DIR/minizinc -v -s --time-limit 60000 --json-stream $MODEL_FILE $CORE_PARAMS_FILE $KAPPA_PARAMS_FILE $OTHER_PARAMS_FILE $RELATIONS_FILE $VECTOR_PARAMS_FILE"
echo "MiniZinc command: $MINIZINC_COMMAND" >&2
eval $MINIZINC_COMMAND > "${PROOF_TAPE_DIR}/full_output.log" 2>&1

MINIZINC_EXIT_CODE=$?

# Display head of stdout.log and stderr.log
echo "--- Head of full_output.log ---"
head -n 20 "${PROOF_TAPE_DIR}/full_output.log"

if [ $MINIZINC_EXIT_CODE -ne 0 ]; then
    echo "MiniZinc model run failed! Check ${PROOF_TAPE_DIR}/full_output.log for details." | tee -a "${PROOF_TAPE_DIR}/error.log"
    exit 1
fi

echo "MiniZinc model run completed. Output saved to ${PROOF_TAPE_DIR}/stdout.log and ${PROOF_TAPE_DIR}/stderr.log"
echo "--- Proof Tape Generation Complete ---"