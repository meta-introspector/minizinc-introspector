#!/bin/bash

# This script automates the process of analyzing your Rust projects
# and finding crates similar to 'file_content_analyzer' based on their code content.

# --- Section 1: Explanation for Noobs ---
# What this script does:
# 1.  **Full Analysis (Building the Brain's Memory)**:
#     - It scans all your Rust projects (folders with a 'Cargo.toml' file) in the
#       '~/storage/github/' directory.
#     - For each Rust file (.rs), it counts words and creates a "bag of words"
#       (a list of all words and how often they appear).
#     - It saves all this information into a special file called 'file_analysis_cache.json'.
#       This file acts like the program's memory, so it can resume if interrupted.
#     - This step can take a long time, especially the first time you run it,
#       as it has to read many files.
# 2.  **Crate Similarity Analysis (Finding Look-Alikes)**:
#     - Once the memory is built, it compares the "bag of words" of the
#       'file_content_analyzer' crate with all other Rust crates it found.
#     - It then tells you which crates are most similar, based on the words they use.

# How to run this script:
# 1.  Open your terminal (Termux).
# 2.  Navigate to the directory where this script is saved:
#     cd /data/data/com.termux/files/home/storage/github/libminizinc/
# 3.  Make the script executable (you only need to do this once):
#     chmod +x run_similarity_analysis.sh
# 4.  Run the script:
#     ./run_similarity_analysis.sh

# --- Section 2: Configuration ---
# Log file for the full analysis step
FULL_ANALYSIS_LOG="full_analysis.log"
# Report file for the crate similarity results
SIMILARITY_REPORT="crate_similarity_report.txt"

# --- Section 3: Execution ---

echo "Starting Full Analysis of Rust Projects..."
echo "This might take a while. Progress will be logged to ${FULL_ANALYSIS_LOG}"

# Step 1: Run the full analysis to build/update the cache
# We redirect stderr to stdout (2>&1) and then pipe all output to tee
# tee allows us to see the output in the terminal AND save it to a file.
# The -- --mode full_analysis tells our Rust program to run in full analysis mode.

cargo run --package file_content_analyzer -- --mode full_analysis 2>&1 | tee "${FULL_ANALYSIS_LOG}"

# Check if the full analysis was successful
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "\nERROR: Full analysis failed. Please check ${FULL_ANALYSIS_LOG} for details."
    exit 1
fi

echo "\nFull Analysis Complete. Cache updated in file_analysis_cache.json."

echo "\nStarting Crate Similarity Analysis..."
echo "Results will be saved to ${SIMILARITY_REPORT}"

# Step 2: Run the crate similarity analysis
# This mode reads from the cache created in Step 1.
# The output (similar crates) will be saved to the report file.

cargo run --package file_content_analyzer -- --mode crate_similarity > "${SIMILARITY_REPORT}"

# Check if the similarity analysis was successful
if [ $? -ne 0 ]; then
    echo "\nERROR: Crate similarity analysis failed. Please check your setup."
    exit 1
fi

echo "\nCrate Similarity Analysis Complete."
echo "You can view the results in: ${SIMILARITY_REPORT}"
echo "To view the full analysis logs, check: ${FULL_ANALYSIS_LOG}"

# --- Section 4: How to View Results ---
# To see the top similar crates, you can use:
# cat ${SIMILARITY_REPORT}

# To see the detailed cache (program's memory), you can use:
# cat file_analysis_cache.json | less

# To see the full analysis progress and any errors, you can use:
# cat ${FULL_ANALYSIS_LOG}
