#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool boollit_get_value(MiniZinc::BoolLit* boollit_ptr) {
    MiniZinc::BoolLit* boollit = reinterpret_cast<MiniZinc::BoolLit*>(boollit_ptr);
    if (!boollit) {
        return false; // Or handle error appropriately
    }
    return boollit->v();
}

} // extern "C"
