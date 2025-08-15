#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool expression_is_boollit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr) {
        return false;
    }
    return expr->isa<MiniZinc::BoolLit>();
}

} // extern "C"
