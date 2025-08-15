#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::StringLit* expression_as_stringlit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr || !expr->isa<MiniZinc::StringLit>()) {
        return nullptr;
    }
    return expr->cast<MiniZinc::StringLit>();
}

} // extern "C"
