#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

unsigned int setlit_get_size(MiniZinc::SetLit* setlit_ptr) {
    MiniZinc::SetLit* setlit = reinterpret_cast<MiniZinc::SetLit*>(setlit_ptr);
    if (!setlit) {
        return 0; // Or handle error appropriately
    }
    return setlit->v().size();
}

} // extern "C"
