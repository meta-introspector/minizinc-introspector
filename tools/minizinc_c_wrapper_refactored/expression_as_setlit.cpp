#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::SetLit* expression_as_setlit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr || !expr->isa<MiniZinc::SetLit>(expr)) {
        return nullptr;
    }
    return expr->cast<MiniZinc::SetLit>(expr);
}

} // extern "C"
