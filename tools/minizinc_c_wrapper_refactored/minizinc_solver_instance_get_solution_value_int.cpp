#include "minizinc_opaque_types.h"
#include <minizinc/solver_instance_base.hh>
#include <minizinc/gecode/gecodesolver.hh> // Assuming Gecode is the default solver
#include <minizinc/model.hh>
#include <minizinc/env.hh>
#include <minizinc/ast.hh>
#include <minizinc/expression.hh>

extern "C" {

int minizinc_solver_instance_get_solution_value_int(MiniZinc::SolverInstanceBase* si_ptr, const char* var_name) {
    MiniZinc::SolverInstanceImpl<MiniZinc::GecodeSolver>* si_impl = 
        reinterpret_cast<MiniZinc::SolverInstanceImpl<MiniZinc::GecodeSolver>*>(si_ptr);

    // Get the environment from the solver instance
    MiniZinc::Env& env = si_impl->env();

    // Find the variable declaration by name
    MiniZinc::VarDecl* var_decl = nullptr;
    for (MiniZinc::VarDecl* vd : env.model().vardecls()) {
        if (vd->id().c_str() == std::string(var_name)) {
            var_decl = vd;
            break;
        }
    }

    if (!var_decl) {
        // Variable not found
        return -1; // Or throw an error, or return a specific error code
    }

    // Get the solution value for the variable
    MiniZinc::Expression* expr = si_impl->getSolutionValue(&var_decl->id());

    if (expr && expr->isIntLit()) {
        return expr->toIntLit()->val();
    } else {
        // Not an integer literal or expression is null
        return -1; // Or throw an error, or return a specific error code
    }
}

} // extern "C"