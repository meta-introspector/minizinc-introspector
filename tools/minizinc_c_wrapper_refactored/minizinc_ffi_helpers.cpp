#include "minizinc_ffi_helpers.h"
#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh>

#ifdef __cplusplus
extern "C" {
#endif

// Helper function to get the MznSolver from MiniZincEnvWrapper
MiniZinc::MznSolver* get_mzn_solver(MiniZincEnvWrapper* env_wrapper) {
    return env_wrapper->solver;
}

// Helper function to create a MiniZincEnvWrapper from an MznSolver
MiniZincEnvWrapper* create_env_wrapper(MiniZinc::MznSolver* solver) {
    MiniZincEnvWrapper* env_wrapper = new MiniZincEnvWrapper();
    env_wrapper->solver = solver;
    // env_wrapper->env is default constructed, no need to explicitly set it unless needed
    return env_wrapper;
}

#ifdef __cplusplus
} // extern "C"
#endif