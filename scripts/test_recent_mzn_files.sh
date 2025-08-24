#!/bin/bash

# Script to test compilation of recent MiniZinc models and report syntax errors.

# Source the environment variables to get LIBMINIZINC_BUILD_DIR
source "$(dirname "$0")"/../.env

MINIZINC_EXECUTABLE="${LIBMINIZINC_BUILD_DIR}/minizinc"

# List of recent .mzn files (replace with actual git log output or dynamic generation)
# For now, I'll hardcode the list based on the last git log output and my recent changes.
# In a real CI/CD, this list would be dynamically generated.
RECENT_MZN_FILES=(
    "gemini_self_model.mzn"
    "narrative_journey.mzn"
    "solfunmeme_vial_engine.mzn"
    "project_model.mzn"
    "combinatorial_topologies.mzn"
    "deep_bootstrap_chain.mzn"
    "development_path_optimizer.mzn"
    "universal_bootstrap_godel.mzn"
    "minizinc_data/chunk0_output/data_declarations.mzn"
    "minizinc_models/constraints.mzn"
    "minizinc_models/solve_output.mzn"
    "word_embedding_inference.mzn"
    "docs/technical/term_recognition_system_generation.mzn"
    "docs/sops/term_recognition_system_generation.mzn"
    "minizinc_output_single_file/ast_model.mzn"
    "minizinc_data/data_declarations.mzn"
    "test_data_include.mzn"
    "test_num_words.mzn"
    "temp_project/minizinc_output/ast_model.mzn"
    "minizinc_models/functions.mzn"
    "test_dzn_word_embeddings.mzn"
    "minizinc_models/search_results_model.mzn"
    "minizinc_models/project_reflection_model.mzn"
    "minizinc_models/zos_gpu_reflection_model.mzn"
    "minizinc_models/project_decomposition_model.mzn"
    "minizinc_models/token_regeneration_demo.mzn"
    "poem.mzn"
    "naming_solver.mzn"
    "minizinc_models/select_needed_constants.mzn"
    "minizinc_models/extract_constants.mzn"
    "test_float_function.mzn" # Added manually
    "minizinc_lessons.mzn"    # Added manually
)

echo "--- Compiling Recent MiniZinc Models ---"
echo "MiniZinc Executable: ${MINIZINC_EXECUTABLE}"
echo ""

SUCCESS_COUNT=0
FAILURE_COUNT=0

for file in "${RECENT_MZN_FILES[@]}"; do
    FULL_PATH="${MINIZINC_PROJECT_ROOT}/${file}"
    echo "Checking: ${file}"
    if [ ! -f "${FULL_PATH}" ]; then
        echo "  [SKIP] File not found: ${FULL_PATH}"
        continue
    fi

    # Use --compile-only to check for syntax errors without solving
    # Redirect stderr to stdout to capture all output
    output=$("${MINIZINC_EXECUTABLE}" -c --solver Gecode "${FULL_PATH}" 2>&1)
    sleep 1
    exit_code=$?

    if [ ${exit_code} -eq 0 ]; then
        echo "  [OK] Compiled successfully."
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        echo "  [ERROR] Compilation failed!"
        echo "--- MiniZinc Output for ${file} ---"
        echo "${output}"
        echo "-----------------------------------"
        FAILURE_COUNT=$((FAILURE_COUNT + 1))
    fi
    echo ""
done

echo "--- Summary ---"
echo "Successfully compiled: ${SUCCESS_COUNT}"
echo "Failed to compile: ${FAILURE_COUNT}"

if [ ${FAILURE_COUNT} -gt 0 ]; then
    echo "Some MiniZinc models failed to compile. Please review the errors above."
    exit 1
else
    echo "All recent MiniZinc models compiled successfully."
    exit 0
fi