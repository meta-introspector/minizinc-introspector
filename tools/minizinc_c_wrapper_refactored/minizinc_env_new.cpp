#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh> // Include MznSolver
#include <iostream>
#include <string>
#include <chrono> // For Timer

extern "C" {

// Change return type to MiniZincEnvWrapper*
MiniZincEnvWrapper* minizinc_env_new() {
    // Allocate Timer on the heap
    MiniZinc::Timer* timer = new MiniZinc::Timer();

    // Allocate MznSolver on the heap, passing the Timer reference
    MiniZinc::MznSolver* solver = new MiniZinc::MznSolver(std::cout, std::cerr, *timer);
    std::cerr << "DEBUG: minizinc_env_new - Created MznSolver at: " << solver << std::endl; std::cerr.flush();

    // Set verbose flag if needed (MznSolver has its own flagVerbose)
    solver->flagVerbose = true;

    // Create and return MiniZincEnvWrapper
    MiniZincEnvWrapper* wrapper = new MiniZincEnvWrapper();
    wrapper->solver = solver;
    wrapper->timer = timer;

    return wrapper;
}

} // extern "C"