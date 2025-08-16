#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh>

// global_env_wrapper is declared in minizinc_ffi_helpers.h

// Declare the LLVM profiling function
extern "C" void __llvm_profile_write_file(const char* Filename);

extern "C" {

void minizinc_env_free(MiniZincEnvWrapper* env_wrapper) {
    // Explicitly write LLVM profile data before freeing the solver
    // This is a debugging step to ensure data is flushed.
    __llvm_profile_write_file("default.profraw");

    delete env_wrapper->solver;
    delete env_wrapper;
    global_env_wrapper = nullptr; // Reset the global pointer
}

} // extern "C"