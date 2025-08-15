#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool expression_is_intlit(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    return MiniZinc::Expression::isa<MiniZinc::IntLit>(expr);
}

} // extern "C"
