#ifndef MINIZINC_FFI_HELPERS_H
#define MINIZINC_FFI_HELPERS_H

#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh>

#ifdef __cplusplus
extern "C" {
#endif

// Helper function to get the MznSolver from MiniZincEnvWrapper
MiniZinc::MznSolver* get_mzn_solver(MiniZincEnvWrapper* env_wrapper);

// Helper function to create a MiniZincEnvWrapper from an MznSolver
MiniZincEnvWrapper* create_env_wrapper(MiniZinc::MznSolver* solver);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // MINIZINC_FFI_HELPERS_H