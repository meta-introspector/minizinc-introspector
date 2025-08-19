#!/bin/bash

# This script runs zos-bootstrap for minimal_test_with_var.rs through the full MiniZinc pipeline.
# It will generate ast_model.mzn and ast_data.dzn in minizinc_output_single_file/.

cargo run --package zos-bootstrap -- test-ast-to-mini-zinc --file-path /data/data/com.termux/files/home/storage/github/libminizinc/minimal_test_with_var.rs
