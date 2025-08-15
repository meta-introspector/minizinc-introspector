#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Define variables
BUILD_DIR="build"
C_TEST_DIR="c_abi_test"
RUST_CRATE_NAME="minizinc_ffi"
MINIZINC_C_WRAPPER_LIB_PATH="${BUILD_DIR}/libminizinc_c_wrapper.so"
MINIZINC_C_WRAPPER_INCLUDE_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper_refactored"

echo "--- Building C++ wrapper ---"
cmake --build ${BUILD_DIR}/ -- -j10

echo "--- Preparing C ABI standalone test ---"
mkdir -p ${C_TEST_DIR}

# Create a simple C test file
cat << EOF > ${C_TEST_DIR}/test_c_abi.cpp
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include the generated FFI header
#include "minizinc_ffi_declarations_v2.h"
#include "minizinc_opaque_types.h" // For MiniZincEnvWrapper

int main() {
    printf("C ABI Test: Starting\n");

    // 1. Create a new MiniZinc environment
    MiniZincEnvWrapper* env_wrapper = minizinc_env_new();
    if (env_wrapper == NULL) {
        fprintf(stderr, "C ABI Test: Failed to create MiniZinc environment\n");
        return 1;
    }
    printf("C ABI Test: MiniZinc environment created at %p\n", (void*)env_wrapper);

    // 2. Parse a simple model
    const char* model_str = "var int: x; constraint x > 0; solve satisfy;";
    MiniZincModel* model = minizinc_parse_string_only(env_wrapper, model_str);
    if (model == NULL) {
        fprintf(stderr, "C ABI Test: Failed to parse model\n");
        minizinc_env_free(env_wrapper);
        return 1;
    }
    printf("C ABI Test: Model parsed successfully at %p\n", (void*)model);

    // 3. Free the model and environment
    minizinc_model_free(model);
    printf("C ABI Test: Model freed.\n");
    minizinc_env_free(env_wrapper);
    printf("C ABI Test: MiniZinc environment freed.\n");

    printf("C ABI Test: All C ABI tests passed successfully.\n");
    return 0;
}
EOF

echo "--- Compiling C ABI standalone test ---"
# Compile the C test program
g++ -o ${C_TEST_DIR}/test_c_abi ${C_TEST_DIR}/test_c_abi.cpp \
    -I${MINIZINC_C_WRAPPER_INCLUDE_DIR} \
    -I/data/data/com.termux/files/home/storage/github/libminizinc/include \
    -I${BUILD_DIR}/include \
    -L${BUILD_DIR} \
    -lminizinc_c_wrapper \
    -Wl,-rpath=${BUILD_DIR} # Add rpath for runtime linking

echo "--- Running C ABI standalone test ---"
${C_TEST_DIR}/test_c_abi

echo "--- Running Rust interface tests ---"
cp ${MINIZINC_C_WRAPPER_LIB_PATH} tools/${RUST_CRATE_NAME}/target/debug/
cargo test --package ${RUST_CRATE_NAME}

echo "--- All build and tests completed successfully ---"