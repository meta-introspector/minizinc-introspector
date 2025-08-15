# Define the minizinc_c_wrapper library
add_library(minizinc_c_wrapper SHARED
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper/minizinc_c_wrapper.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/arraylit_get_element_at_index.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/arraylit_get_size.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/boollit_get_value.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_as_anon_var.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_as_arraylit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_as_boollit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_as_floatlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_as_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_as_setlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_as_stringlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_get_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_anon_var.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_arraylit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_boollit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_floatlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_intlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_setlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_stringlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/floatlit_get_value.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/id_get_value.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_as_assign.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_as_constraint.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_as_function.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_as_include.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_as_vardecl.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_get_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_is_assign.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_is_constraint.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_is_function.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_is_include.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_is_vardecl.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_env_free.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_env_new.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_ffi_helpers.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_gc_lock.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_gc_unlock.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_env_wrapper.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_get_executable_path.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_get_version_string.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_model_doc_comment.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_model_free.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_model_get_output_item.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_model_get_parent.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_model_get_solve_item.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_parse_model_v2.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_parse_model_with_flags.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_parse_string_only.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_solver_free.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_solver_get_solution_value_int.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_solver_get_solver_instance.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_solver_instance_next.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_solver_instance_print_solution.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_solver_run.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_string_free.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_filename.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_filepath.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_item_at_index.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_num_items.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/setlit_get_element_at_index.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/setlit_get_size.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/stringlit_get_value.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/test_ffi_parser.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_get_base_type.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_ann.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_bool_array.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_bool_set.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_bool.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_bot.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_float_set.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_float.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_int_array.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_int_set_array.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_int_set.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_int.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_opt.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_par.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_plain.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_present.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_set.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_string.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_top.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_unknown.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_is_var.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_get_expression.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_get_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_get_payload.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_get_type_inst.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_is_evaluated.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_is_introduced.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_is_toplevel.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_is_type_alias.cpp
)
include(cmake/targets/minizinc_expression_sources.cmake)

# Link against libmzn (the main MiniZinc library)
target_link_libraries(minizinc_c_wrapper PRIVATE mzn)

# Set include directories for the C wrapper
target_include_directories(minizinc_c_wrapper PRIVATE
    ${PROJECT_SOURCE_DIR}/include
    ${PROJECT_BINARY_DIR}/include
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper
)

# Set RPATH for the C wrapper to find libmzn.so at runtime
# This is crucial for dynamic linking on Linux/Android
target_link_options(minizinc_c_wrapper PRIVATE
    -Wl,-rpath,\$ORIGIN/../lib # Relative to the shared library itself
    -Wl,-rpath,${CMAKE_INSTALL_PREFIX}/lib # Absolute path for installation
)

# Install the C wrapper library
install(TARGETS minizinc_c_wrapper
    DESTINATION ${CMAKE_INSTALL_LIBDIR}
)