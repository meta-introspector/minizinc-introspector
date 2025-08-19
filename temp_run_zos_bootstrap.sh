#!/bin/bash

PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"
export LD_LIBRARY_PATH="${PROJECT_ROOT}/tools/minizinc_c_wrapper/:${PROJECT_ROOT}/install/lib/"

"${PROJECT_ROOT}/target/release/zos-bootstrap" ast-to-mini-zinc --single-file-path "${PROJECT_ROOT}/rust_lattice_project/src/nodes/node_01_empty_main.rs" --output-dir "${PROJECT_ROOT}/minizinc_lattice_output/node_01"