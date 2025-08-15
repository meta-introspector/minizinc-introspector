#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

double floatlit_get_value(FloatLit* floatlit_ptr) {
    MiniZinc::FloatLit* floatlit = reinterpret_cast<MiniZinc::FloatLit*>(floatlit_ptr);
    return MiniZinc::FloatLit::v(floatlit).toDouble();
}

} // extern "C"