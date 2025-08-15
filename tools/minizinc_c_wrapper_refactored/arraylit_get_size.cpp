#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

unsigned int arraylit_get_size(MiniZinc::ArrayLit* arraylit_ptr) {
    MiniZinc::ArrayLit* arraylit = reinterpret_cast<MiniZinc::ArrayLit*>(arraylit_ptr);
    if (!arraylit) {
        return 0; // Or handle error appropriately
    }
    return arraylit->size();
}

} // extern "C"
