#!/bin/bash

export MINIZINC_PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"

# Driver script to run all MiniZinc tests and report on time per script.

# Source the environment variables
source "${MINIZINC_PROJECT_ROOT}/.env"

# Find all .mzn files in the minizinc_models directory
MZN_FILES=$(find "${MINIZINC_MODELS_DIR}" -name "*.mzn")

# Array to store test results
TEST_RESULTS=()

# Function to determine data files for a given model
get_data_files() {
  local model_name=$(basename "$1" .mzn)
  local data_files=""

  case "$model_name" in
    "partitioned_manifold")
      data_files="${MINIZINC_DATA_DIR}/partitioned_manifold_minimal.dzn"
      ;;
    "test_unit_norm")
      data_files="${MINIZINC_DATA_DIR}/partitioned_manifold_minimal.dzn"
      ;; 
    # Add more cases for models that require specific .dzn files
    # "your_model_name")
    #   data_files="${MINIZINC_DATA_DIR}/your_data.dzn"
    #   ;;
  esac
  echo "$data_files"
}

# Iterate through each .mzn file
for MZN_FILE in $MZN_FILES; 
do
  MODEL_NAME=$(basename "$MZN_FILE" .mzn)
  echo "\n--- Running test for $MODEL_NAME ---"

  DATA_FILES=$(get_data_files "$MZN_FILE")

  # Run the individual test script
  ./scripts/run_minizinc_test.sh "$MZN_FILE" "$DATA_FILES"

  # Read the duration from the time.log file
  TIME_FILE="${MINIZINC_PROJECT_ROOT}/tests/minizinc_tests/${MODEL_NAME}/time.log"
  DURATION="N/A"
  if [ -f "$TIME_FILE" ]; then
    DURATION=$(cat "$TIME_FILE" | grep "Duration:" | awk '{print $2, $3}')
  fi

  TEST_RESULTS+=("$MODEL_NAME: $DURATION")

done

echo "\n--- All Tests Completed ---"
echo "\n--- Test Summary ---"
for RESULT in "${TEST_RESULTS[@]}"; 
do
  echo "$RESULT"
done
