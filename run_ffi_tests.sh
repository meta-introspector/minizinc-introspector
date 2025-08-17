#!/bin/bash

PROJECT_ROOT="/data/data/com.termux/files/home/storage/github/libminizinc"
export LD_LIBRARY_PATH="${PROJECT_ROOT}/tools/minizinc_c_wrapper/:${PROJECT_ROOT}/install/lib/"

cargo test --package minizinc_ffi
