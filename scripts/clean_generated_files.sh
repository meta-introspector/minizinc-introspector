#!/bin/bash
# Cleans generated term chunks and DFA modules

# Get project root from build_config.toml (assuming it's at libminizinc root)
PROJECT_ROOT=$(grep -E '^project_root' build_config.toml | cut -d'=' -f2 | tr -d ' "')

# Remove zos-fast-query generated files
rm -rf "${PROJECT_ROOT}target/debug/build/zos-fast-query-*/out/"
rm -rf "${PROJECT_ROOT}target/release/build/zos-fast-query-*/out/"

# Remove vocabulary_dfa_lib generated subdirectories
# A safer approach might be to list directories and filter
find "${PROJECT_ROOT}crates/vocabulary_dfa_lib/src/" -maxdepth 1 -type d -name '[a-z]' -exec rm -rf {} + 
find "${PROJECT_ROOT}crates/vocabulary_dfa_lib/src/" -maxdepth 1 -type d -name '[0-9]' -exec rm -rf {} + 
find "${PROJECT_ROOT}crates/vocabulary_dfa_lib/src/" -maxdepth 1 -type d -name 'U*' -exec rm -rf {} + 

echo "Cleaned generated files."
