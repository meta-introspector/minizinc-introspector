# Define the minizinc_c_wrapper library
add_library(minizinc_c_wrapper SHARED
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper/minizinc_c_wrapper.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_env_wrapper.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_env_new.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_env_free.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_parse_model.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_parse_data_from_string.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_model_free.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_get_version_string.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_filename.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_filepath.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_num_items.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/model_get_item_at_index.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_get_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_is_vardecl.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/item_as_vardecl.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_get_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_get_type_inst.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/vardecl_get_expression.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/typeinst_get_base_type.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_get_id.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/expression_is_intlit.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_get_mznlib_dir.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_get_executable_path.cpp
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_model_doc_comment.cpp
)

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