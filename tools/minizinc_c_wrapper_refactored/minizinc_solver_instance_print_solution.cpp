#include "minizinc_opaque_types.h"
#include <minizinc/solver_instance_base.hh>

extern "C" {

void minizinc_solver_instance_print_solution(MiniZinc::SolverInstanceBase* solver_instance_ptr) {
    MiniZinc::SolverInstanceBase* si = reinterpret_cast<MiniZinc::SolverInstanceBase*>(solver_instance_ptr);
    si->printSolution();
}

} // extern "C"