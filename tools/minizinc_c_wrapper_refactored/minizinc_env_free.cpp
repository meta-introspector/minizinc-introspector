#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh>

extern "C" {

void minizinc_env_free(MiniZincEnvWrapper* env_wrapper) {
    
    delete env_wrapper->solver;
    delete env_wrapper;
}

} // extern "C"