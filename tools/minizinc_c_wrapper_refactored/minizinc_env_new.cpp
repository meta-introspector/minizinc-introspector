#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh> // Include MznSolver
#include <iostream>
#include <string>
#include <chrono> // For Timer
#include <stdexcept> // For std::runtime_error
#include <cassert> // For assert

// Static variable to track if the environment has been initialized
MiniZincEnvWrapper* global_env_wrapper = nullptr;

extern "C" {

MiniZincEnvWrapper* minizinc_env_new() {
    MiniZinc::GCLock lock; // Acquire GC lock for this function

    if (global_env_wrapper != nullptr) {
        std::cerr << "ERROR: minizinc_env_new() called more than once. This is not allowed." << std::endl; std::cerr.flush();
        assert(false); // Crash if called more than once
    }
    
    // Use a static Timer to ensure it's allocated once and lives for the program's duration.
    // This prevents a memory leak as MznSolver expects a Timer reference.
    static MiniZinc::Timer timer; 

    // Allocate MznSolver on the heap, passing the static Timer reference
    MiniZinc::MznSolver* solver = new MiniZinc::MznSolver(std::cout, std::cerr, timer);
    std::cerr << "DEBUG: minizinc_env_new - Created MznSolver at: " << solver << std::endl; std::cerr.flush();

    // Set verbose flag if needed (MznSolver has its own flagVerbose)
    solver->flagVerbose = true;

    // Return the MznSolver directly
    MiniZincEnvWrapper* env_wrapper = new MiniZincEnvWrapper();
    env_wrapper->solver = solver;
    
    global_env_wrapper = env_wrapper; // Store the newly created environment globally

    return env_wrapper;
}

} // extern "C"