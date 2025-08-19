#!/bin/bash

# This script specifically tests the crate similarity analysis for 'file_content_analyzer'.

# --- Section 1: Explanation ---
# What this script does:
# - It runs the 'file_content_analyzer' program in 'crate_similarity' mode.
# - It specifically asks the program to compare against the 'file_content_analyzer' crate itself.
# - It requests the top 10 most similar crates.
# - The results will be saved to 'test_crate_similarity_report.txt'.

# --- Section 2: How to Run ---
# 1.  Ensure you have run the full analysis at least once to generate the necessary cache files:
#     ./run_similarity_analysis.sh
#     (This will create/update the .file_analysis_summary.json files for each project)
# 2.  Make this script executable (you only need to do this once):
#     chmod +x test_crate_similarity.sh
# 3.  Run the script:
#     ./test_crate_similarity.sh

# --- Section 3: Execution ---

REPORT_FILE="test_crate_similarity_report.txt"

echo "Running specific crate similarity test for 'file_content_analyzer'..."
echo "Results will be saved to ${REPORT_FILE}"

cargo run --package file_content_analyzer -- --mode crate_similarity --target-crate file_content_analyzer --most-similar 10 > "${REPORT_FILE}"

if [ $? -ne 0 ]; then
    echo "\nERROR: Specific crate similarity test failed. Please check the output."
    exit 1
fi

echo "\nTest complete. Results in: ${REPORT_FILE}"

# --- Section 4: View Results ---
# To view the results, use:
# cat ${REPORT_FILE}
