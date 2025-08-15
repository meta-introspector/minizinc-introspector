#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool expression_is_anon_var(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr) {
        return false;
    }
    return expr->isa<MiniZinc::AnonVar>(expr);
}

} // extern "C"
