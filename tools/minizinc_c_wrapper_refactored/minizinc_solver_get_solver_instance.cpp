#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh>
#include <minizinc/solver_instance_base.hh>

extern "C" {

MiniZinc::SolverInstanceBase* minizinc_solver_get_solver_instance(MiniZinc::MznSolver* solver_ptr) {
    MiniZinc::MznSolver* solver = solver_ptr;
    return solver->getSI();
}

} // extern "C"