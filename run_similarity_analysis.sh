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
#     - It saves all this information into individual files named '.file_analysis_summary.json'
#       within each project's root directory. This acts like the program's memory,
#       allowing it to resume if interrupted and only re-process changed files.
#     - This step also generates 'term_index.json' for keyword searches and
#       'file_pair_similarities.json' for file-level similarity details.
#     - This step can take a long time, especially the first time you run it,
#       as it has to read many files.
# 2.  **Crate Similarity Analysis (Finding Look-Alikes)**:
#     - Once the memory is built, it compares the "bag of words" of a specified
#       crate (by default, 'file_content_analyzer') with all other Rust crates it found.
#     - It then tells you which crates are most similar, based on the words they use.
#       You can specify which crate to compare against and how many top results to show.

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
# Report file for the stopword candidates
STOPWORD_REPORT="stopword_report.txt"

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

echo "\nFull Analysis Complete. Cache updated in individual .file_analysis_summary.json files."

echo "\nStarting Crate Similarity Analysis..."
echo "Results will be saved to ${SIMILARITY_REPORT}"

# Step 2: Run the crate similarity analysis
# This mode reads from the per-project summaries created in Step 1.
# We are specifically asking for the top 10 most similar crates to 'file_content_analyzer'.

cargo run --package file_content_analyzer -- --mode crate_similarity --target-crate file_content_analyzer --most-similar 10 > "${SIMILARITY_REPORT}"

# Check if the similarity analysis was successful
if [ $? -ne 0 ]; then
    echo "\nERROR: Crate similarity analysis failed. Please check your setup."
    exit 1
fi

echo "\nCrate Similarity Analysis Complete."
echo "You can view the results in: ${SIMILARITY_REPORT}"

# --- Section 4: Additional Analysis Examples (Optional) ---

echo "\n--- Optional: Generating Stopword Report ---"
echo "This will identify words that appear in a high percentage of files."
echo "Results will be saved to ${STOPWORD_REPORT}"
# To run this, uncomment the following lines:
# cargo run --package file_content_analyzer -- --mode generate_stopword_report > "${STOPWORD_REPORT}"
# if [ $? -ne 0 ]; then
#     echo "\nERROR: Stopword report generation failed."
# fi
# echo "Stopword report available in: ${STOPWORD_REPORT}"

echo "\n--- Optional: Searching for Keywords ---"
echo "This will search for specific keywords across your codebase."
# To run this, uncomment the following lines and replace 'keyword1' 'keyword2' with your terms:
# cargo run --package file_content_analyzer -- --mode search_keywords --keywords keyword1 keyword2 > search_results.txt
# if [ $? -ne 0 ]; then
#     echo "\nERROR: Keyword search failed."
# fi
# echo "Keyword search results available in: search_results.txt"

# --- Section 5: How to View Results ---
# To see the top similar crates, you can use:
# cat "${SIMILARITY_REPORT}"

# To see the stopword report, you can use:
# cat "${STOPWORD_REPORT}"

# To see the full analysis logs, check:
# cat "${FULL_ANALYSIS_LOG}"

# To view the detailed per-project cache (program's memory), look for .file_analysis_summary.json files
# within your project directories (e.g., find ~/storage/github/ -name ".file_analysis_summary.json")

# To view the term index (for keyword search), you can use:
# cat ~/storage/github/term_index.json | less

# To view the file pair similarities, you can use:
# cat ~/storage/github/file_pair_similarities.json | less