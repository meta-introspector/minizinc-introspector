#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh>

extern "C" {

void minizinc_solver_free(MiniZinc::MznSolver* solver_ptr) {
    MiniZinc::MznSolver* solver = reinterpret_cast<MiniZinc::MznSolver*>(solver_ptr);
    delete solver;
}

} // extern "C"