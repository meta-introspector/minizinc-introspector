#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool expression_is_floatlit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr) {
        return false;
    }
    return expr->isa<MiniZinc::FloatLit>();
}

} // extern "C"
