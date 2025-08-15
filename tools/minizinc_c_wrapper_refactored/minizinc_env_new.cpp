#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh> // Include MznSolver
#include <iostream>
#include <string>
#include <chrono> // For Timer
#include <stdexcept> // For std::runtime_error

extern "C" {

// Change return type to MiniZincEnvWrapper*
MiniZincEnvWrapper* minizinc_env_new() {
    // Always throw an exception to identify all callers
    throw std::runtime_error("minizinc_env_new() was called. This is for debugging purposes.");

    // The rest of the original code is unreachable now, but I'll keep it commented out
    // for reference if needed later.
    /*
    MiniZinc::GCLock lock; // Acquire GC lock for this function
    static MiniZinc::Timer timer; 

    MiniZinc::MznSolver* solver = new MiniZinc::MznSolver(std::cout, std::cerr, timer);
    std::cerr << "DEBUG: minizinc_env_new - Created MznSolver at: " << solver << std::endl; std::cerr.flush();

    solver->flagVerbose = true;

    MiniZincEnvWrapper* env_wrapper = new MiniZincEnvWrapper();
    env_wrapper->solver = solver;
    return env_wrapper;
    */
}

} // extern "C"