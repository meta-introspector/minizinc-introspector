# Define the minizinc_c_wrapper library
add_library(minizinc_c_wrapper SHARED
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper/minizinc_c_wrapper.cpp
)

# Link against libmzn (the main MiniZinc library)
target_link_libraries(minizinc_c_wrapper PRIVATE mzn)

# Set include directories for the C wrapper
target_include_directories(minizinc_c_wrapper PRIVATE
    ${PROJECT_SOURCE_DIR}/include
    ${PROJECT_BINARY_DIR}/include
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