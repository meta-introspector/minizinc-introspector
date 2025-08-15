#include "minizinc_opaque_types.h"
#include <minizinc/solver_instance_base.hh>

extern "C" {

int minizinc_solver_instance_next(MiniZinc::SolverInstanceBase* si_ptr) {
    MiniZinc::SolverInstanceBase* si = reinterpret_cast<MiniZinc::SolverInstanceBase*>(si_ptr);
    return static_cast<int>(si->next());
}

} // extern "C"