#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::AnonVar* expression_as_anon_var(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    if (!expr || !expr->isa<MiniZinc::AnonVar>()) {
        return nullptr;
    }
    return expr->cast<MiniZinc::AnonVar>();
}

} // extern "C"
