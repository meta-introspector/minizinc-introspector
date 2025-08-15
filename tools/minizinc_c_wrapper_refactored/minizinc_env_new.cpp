#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh> // Include MznSolver
#include <iostream>
#include <string>
#include <chrono> // For Timer

extern "C" {

// Change return type to MiniZincEnvWrapper*
MiniZinc::MznSolver* minizinc_env_new() {
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
    // env_wrapper->env is default constructed, no need to explicitly set it unless needed
    return reinterpret_cast<MiniZinc::MznSolver*>(env_wrapper);
}

} // extern "C"