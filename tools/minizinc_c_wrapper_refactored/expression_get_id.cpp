#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

int expression_get_id(MiniZinc::Expression* expr_ptr) {
    MiniZinc::Expression* expr = reinterpret_cast<MiniZinc::Expression*>(expr_ptr);
    return static_cast<int>(MiniZinc::Expression::eid(expr));
}

} // extern "C"
