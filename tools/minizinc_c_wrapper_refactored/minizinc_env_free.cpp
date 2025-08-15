#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh> // Include MznSolver
#include <iostream>

extern "C" {

void minizinc_env_free(MiniZinc::MznSolver* env_ptr) {
    std::cerr << "DEBUG: minizinc_env_free - Freeing MznSolver at: " << env_ptr << std::endl; std::cerr.flush();
    delete env_ptr;
}

} // extern "C"
