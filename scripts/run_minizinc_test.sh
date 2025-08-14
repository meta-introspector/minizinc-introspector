#!/bin/bash

# Generic script to run a MiniZinc model and capture time and output.

# Source the environment variables
source "${MINIZINC_PROJECT_ROOT}/.env"

# Check if model file is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <model_file_path> [data_file_path1] [data_file_path2] ..."
    echo "Example: $0 minizinc_models/test.mzn"
    echo "Example: $0 minizinc_models/embedding_sphere.mzn minizinc_data/example.dzn"
    exit 1
fi

MODEL_PATH="$1"
shift # Remove model path from arguments, leaving only data paths
DATA_PATHS="$@"

MODEL_NAME=$(basename "$MODEL_PATH" .mzn)
TEST_OUTPUT_DIR="${MINIZINC_PROJECT_ROOT}/tests/minizinc_tests/${MODEL_NAME}"
mkdir -p "$TEST_OUTPUT_DIR"

OUTPUT_FILE="${TEST_OUTPUT_DIR}/stdout.log"
ERROR_FILE="${TEST_OUTPUT_DIR}/stderr.log"
TIME_FILE="${TEST_OUTPUT_DIR}/time.log"

echo "--- Running MiniZinc Model: $MODEL_PATH ---"
echo "Output will be saved to: $TEST_OUTPUT_DIR"

MINIZINC_COMMAND="${LIBMINIZINC_BUILD_DIR}/minizinc --time-limit 6000 -s ${MODEL_PATH} ${DATA_PATHS}"

echo "MiniZinc command: ${MINIZINC_COMMAND}" >&2

# Execute MiniZinc command and capture time, stdout, and stderr
START_TIME=$(date +%s.%N)

${MINIZINC_COMMAND} > "${OUTPUT_FILE}" 2> "${ERROR_FILE}"

END_TIME=$(date +%s.%N)

DURATION=$(echo "$END_TIME - $START_TIME" | bc)

echo "Duration: ${DURATION} seconds" > "${TIME_FILE}"

MINIZINC_EXIT_CODE=$?

if [ $MINIZINC_EXIT_CODE -eq 0 ]; then
  echo "MiniZinc run completed successfully for $MODEL_NAME."
else
  echo "MiniZinc run failed for $MODEL_NAME (Exit Code: $MINIZINC_EXIT_CODE)."
fi

echo "--- Output for $MODEL_NAME ---"
cat "$OUTPUT_FILE"

echo "--- Errors for $MODEL_NAME ---"
cat "$ERROR_FILE"

echo "--- Time for $MODEL_NAME ---"
cat "$TIME_FILE"

exit $MINIZINC_EXIT_CODE