#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::ArrayLit* expression_as_arraylit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr || !expr->isa<MiniZinc::ArrayLit>(expr)) {
        return nullptr;
    }
    return expr->cast<MiniZinc::ArrayLit>(expr);
}

} // extern "C"
