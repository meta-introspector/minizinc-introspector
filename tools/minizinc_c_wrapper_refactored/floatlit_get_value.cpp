#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

double floatlit_get_value(MiniZinc::FloatLit* floatlit_ptr) {
    MiniZinc::FloatLit* floatlit = reinterpret_cast<MiniZinc::FloatLit*>(floatlit_ptr);
    if (!floatlit) {
        return 0.0; // Or handle error appropriately
    }
    return floatlit->v();
}

} // extern "C"
