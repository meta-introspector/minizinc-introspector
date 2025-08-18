#!/bin/bash

# Script to run a specific step of the self-optimize command.
# Usage: ./run_self_optimize_step.sh --step <step_name> --source-path <path_to_project>

STEP_NAME=""
SOURCE_PATH=""
REPORT_FILE=""

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --step)
            STEP_NAME="$2"
            shift
            ;;
        --source-path)
            SOURCE_PATH="$2"
            shift
            ;; 
        *)
            echo "Unknown parameter passed: $1"
            exit 1
            ;;
    esac
    shift
done

if [ -z "$STEP_NAME" ] || [ -z "$SOURCE_PATH" ]; then
    echo "Usage: ./run_self_optimize_step.sh --step <step_name> --source-path <path_to_project>"
    exit 1
fi

# Define report file based on step name
REPORT_FILE="temp/self_optimize_report_${STEP_NAME}.txt"

echo "--- Running self-optimize step: $STEP_NAME ---"
echo "Source path: $SOURCE_PATH"
echo "Report file: $REPORT_FILE"

# Execute the command and tee the output
target/debug/temp_zos_test self-optimize \
    --source-path "$SOURCE_PATH" \
    --step "$STEP_NAME" \
    --output-optimization-report "$REPORT_FILE" \
    2>&1 | tee "$REPORT_FILE.log"

echo "--- Step $STEP_NAME completed. Output also saved to $REPORT_FILE.log ---"
