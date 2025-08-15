#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::Id* expression_as_id(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr || !expr->isa<MiniZinc::Id>(expr)) {
        return nullptr;
    }
    return expr->cast<MiniZinc::Id>(expr);
}

} // extern "C"
