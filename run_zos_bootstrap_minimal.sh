#!/bin/bash

# This script runs zos-bootstrap to generate ast_model.mzn and ast_data.dzn for minimal_test.rs.
# The generated files will be located in /data/data/com.termux/files/home/storage/github/libminizinc/minizinc_output_single_file/

cargo run --package zos-bootstrap -- test-ast-to-mini-zinc --file-path /data/data/com.termux/files/home/storage/github/libminizinc/minimal_test.rs
