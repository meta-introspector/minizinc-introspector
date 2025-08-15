#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::BoolLit* expression_as_boollit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr || !expr->isa<MiniZinc::BoolLit>()) {
        return nullptr;
    }
    return expr->cast<MiniZinc::BoolLit>();
}

} // extern "C"
