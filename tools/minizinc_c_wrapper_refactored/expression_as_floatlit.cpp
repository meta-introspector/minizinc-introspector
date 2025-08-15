#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::FloatLit* expression_as_floatlit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr || !expr->isa<MiniZinc::FloatLit>(expr)) {
        return nullptr;
    }
    return expr->cast<MiniZinc::FloatLit>(expr);
}

} // extern "C"
