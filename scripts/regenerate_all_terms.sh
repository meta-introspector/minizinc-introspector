#!/bin/bash
# Regenerates all term data and DFA modules

echo "Regenerating zos-fast-query term data..."
cargo build --package zos-fast-query

echo "Regenerating vocabulary_dfa_modules..."
cargo run --package vocabulary_dfa_generator

echo "All term data and DFA modules regenerated."
