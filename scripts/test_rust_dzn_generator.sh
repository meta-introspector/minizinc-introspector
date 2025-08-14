#!/bin/bash

RUST_DATA_GENERATOR_EXE="/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_data_generator_rs/minizinc_data_generator_rs/target/release/minizinc_data_generator_rs"
SAMPLE_NUM_VEC=3

echo "--- Testing Rust DZN Generator with num_vec = $SAMPLE_NUM_VEC ---"
"$RUST_DATA_GENERATOR_EXE" "$SAMPLE_NUM_VEC"
echo "--- Test Complete ---"