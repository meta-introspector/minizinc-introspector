#!/bin/bash

# This script updates the docs/code_reviews/index.md file
# with entries for all reviewed code files.

INDEX_FILE="/data/data/com.termux/files/home/storage/github/libminizinc/docs/code_reviews/index.md"

# Start with the header
cat <<EOF > "$INDEX_FILE"
# Code Reviews Index

This document indexes the reviews of various code files within the `libminizinc` project.

## Shell Scripts

EOF

# Add shell script entries
cat <<EOF >> "$INDEX_FILE"
*   [run_rust_ffi_tests_for_coverage.sh](shell_scripts/run_rust_ffi_tests_for_coverage_sh_review.md)
*   [generate_llvm_text_summary.sh](shell_scripts/generate_llvm_text_summary_sh_review.md)
*   [generate_llvm_html_report.sh](shell_scripts/generate_llvm_html_report_sh_review.md)
*   [build_minizinc_with_coverage.sh](shell_scripts/build_minizinc_with_coverage_sh_review.md)
*   [build_and_test.sh](shell_scripts/build_and_test_sh_review.md)
*   [build_test_c_rust.sh](shell_scripts/build_test_c_rust_sh_review.md)
*   [generate_ffi_declarations.sh](shell_scripts/generate_ffi_declarations_sh_review.md)
*   [reproduce_crash.sh](shell_scripts/reproduce_crash_sh_review.md)
*   [reproduce_minizinc_ffi_bug.sh](shell_scripts/reproduce_minizinc_ffi_bug_sh_review.md)
*   [build_gecode.sh](shell_scripts/build_gecode_sh_review.md)
*   [build_libminizinc.sh](shell_scripts/build_libminizinc_sh_review.md)
*   [generate_pub_uses.sh](shell_scripts/generate_pub_uses_sh_review.md)
*   [replace_abs_path_prefix.sh](shell_scripts/replace_abs_path_prefix_sh_review.md)
*   [template_ffi_replace.sh](shell_scripts/template_ffi_replace_sh_review.md)
*   [qa_dzn_generation_verification.sh](shell_scripts/qa_dzn_generation_verification_sh_review.md)
*   [run_all_minizinc_tests.sh](shell_scripts/run_all_minizinc_tests_sh_review.md)
*   [run_embedding_model_v6_test.sh](shell_scripts/run_embedding_model_v6_test_sh_review.md)
*   [run_embedding_model_v6.sh](shell_scripts/run_embedding_model_v6_sh_review.md)
*   [run_embedding_model_v7.sh](shell_scripts/run_embedding_model_v7_sh_review.md)
*   [run_minizinc_minimal.sh](shell_scripts/run_minizinc_minimal_sh_review.md)
*   [run_minizinc_test_driver.sh](shell_scripts/run_minizinc_test_driver_sh_review.md)
*   [run_minizinc_test.sh](shell_scripts/run_minizinc_test_sh_review.md)
*   [run_v7_debug_tests.sh](shell_scripts/run_v7_debug_tests_sh_review.md)
*   [setup_minizinc_solvers.sh](shell_scripts/setup_minizinc_solvers_sh_review.md)
*   [test_rust_dzn_generator.sh](shell_scripts/test_rust_dzn_generator_sh_review.md)
*   [ltmain.sh](shell_scripts/ltmain_sh_review.md)

EOF

cat <<EOF >> "$INDEX_FILE"

## Rust FFI Code

EOF

# Add Rust FFI entries
cat <<EOF >> "$INDEX_FILE"
*   [tests/mod.rs](rust_ffi/tests_mod_rs_review.md)
*   [tests/tests/test_06_new_feature.rs](rust_ffi/test_06_new_feature_rs_review.md)
*   [tests/tests/test_05_full_lifecycle_no_filepath.rs](rust_ffi/test_05_full_lifecycle_no_filepath_rs_review.md)
*   [tests/tests/test_04_get_model_filename.rs](rust_ffi/test_04_get_model_filename_rs_review.md)
*   [tests/tests/test_03_unwrap_model.rs](rust_ffi/test_03_unwrap_model_rs_review.md)
*   [tests/tests/test_02_parse_minimal_model.rs](rust_ffi/test_02_parse_minimal_model_rs_review.md)
*   [tests/tests/test_01_init_global_env.rs](rust_ffi/test_01_init_global_env_rs_review.md)
*   [tests/tests/minimal_crash_tests.rs](rust_ffi/minimal_crash_tests_rs_review.md)
*   [tests/tests/parser_tests.rs](rust_ffi/parser_tests_rs_review.md)
*   [array_lit.rs](rust_ffi/array_lit_rs_review.md)
*   [base_type.rs](rust_ffi/base_type_rs_review.md)
*   [bool_lit.rs](rust_ffi/bool_lit_rs_review.md)
*   [coverage_report.rs](rust_ffi/coverage_report_rs_review.md)
*   [environment/impl_drop.rs](rust_ffi/environment_impl_drop_rs_review.md)
*   [environment/impl_get_executable_path.rs](rust_ffi/environment_impl_get_executable_path_rs_review.md)
*   [environment/impl_get_mznlib_dir.rs](rust_ffi/environment_impl_get_mznlib_dir_rs_review.md)
*   [environment/impl_get_solution_value_int.rs](rust_ffi/environment_impl_get_solution_value_int_rs_review.md)
*   [environment/impl_get_solver_instance.rs](rust_ffi/environment_impl_get_solver_instance_rs_review.md)
*   [environment/impl_get_version_string.rs](rust_ffi/environment_impl_get_version_string_rs_review.md)
*   [environment/impl_new.rs](rust_ffi/environment_impl_new_rs_review.md)
*   [environment/impl_parse_data.rs](rust_ffi/environment_impl_parse_data_rs_review.md)
*   [environment/impl_parse_model.rs](rust_ffi/environment_impl_parse_model_rs_review.md)
*   [environment/impl_parse_string.rs](rust_ffi/environment_impl_parse_string_rs_review.md)
*   [environment/impl_solver_instance_next.rs](rust_ffi/environment_impl_solver_instance_next_rs_review.md)
*   [environment/impl_solver_instance_print_solution.rs](rust_ffi/environment_impl_solver_instance_print_solution_rs_review.md)
*   [environment/minizinc_environment_struct.rs](rust_ffi/environment_minizinc_environment_struct_rs_review.md)
*   [environment/mod.rs](rust_ffi/environment_mod_rs_review.md)
*   [expression_id.rs](rust_ffi/expression_id_rs_review.md)
*   [expression.rs](rust_ffi/expression_rs_review.md)
*   [feature_tests/mod.rs](rust_ffi/feature_tests_mod_rs_review.md)
*   [feature_tests/test_basic_int_var.rs](rust_ffi/feature_tests_test_basic_int_var_rs_review.md)
*   [feature_tests/test_empty_model.rs](rust_ffi/feature_tests_test_empty_model_rs_review.md)
*   [feature_tests/test_output_statement.rs](rust_ffi/feature_tests_test_output_statement_rs_review.md)
*   [ffi_bindings.rs](rust_ffi/ffi_bindings_rs_review.md)
*   [float_lit.rs](rust_ffi/float_lit_rs_review.md)
*   [id_impl.rs](rust_ffi/id_impl_rs_review.md)
*   [id.rs](rust_ffi/id_rs_review.md)
*   [item_id.rs](rust_ffi/item_id_rs_review.md)
*   [item.rs](rust_ffi/item_rs_review.md)
*   [lib.rs](rust_ffi/lib_rs_review.md)
*   [model.rs](rust_ffi/model_rs_review.md)
*   [set_lit.rs](rust_ffi/set_lit_rs_review.md)
*   [string_lit_impl.rs](rust_ffi/string_lit_impl_rs_review.md)
*   [string_lit.rs](rust_ffi/string_lit_rs_review.md)
*   [tests/test_parse_model_v2.rs](rust_ffi/test_parse_model_v2_rs_review.md)
*   [typeinst.rs](rust_ffi/typeinst_rs_review.md)
*   [types/mod.rs](rust_ffi/types_mod_rs_review.md)
*   [vardecl.rs](rust_ffi/vardecl_rs_review.md)

EOF

cat <<EOF >> "$INDEX_FILE"

## C++ FFI Code

EOF

# Add C++ FFI entries (summarized)
cat <<EOF >> "$INDEX_FILE"
*   [C++ FFI Summary](cpp_ffi/cpp_ffi_summary.md)

EOF

echo "Index file updated: $INDEX_FILE"