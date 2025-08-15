#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh> // Include MznSolver
#include <iostream>
#include <string>
#include <chrono> // For Timer

extern "C" {

// Change return type to MznSolver*
MiniZinc::MznSolver* minizinc_env_new() {
    // Create a dummy Timer for MznSolver constructor
    MiniZinc::Timer dummy_timer; // MznSolver expects a Timer object

    // Create an MznSolver object
    MiniZinc::MznSolver* solver = new MiniZinc::MznSolver(std::cout, std::cerr, dummy_timer);
    std::cerr << "DEBUG: minizinc_env_new - Created MznSolver at: " << solver << std::endl; std::cerr.flush();

    // Set verbose flag if needed (MznSolver has its own flagVerbose)
    solver->flagVerbose = true;

    return solver;
}

} // extern "C"
