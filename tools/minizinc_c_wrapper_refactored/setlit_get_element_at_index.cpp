#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::Expression* setlit_get_element_at_index(MiniZinc::SetLit* setlit_ptr, unsigned int index) {
    MiniZinc::SetLit* setlit = reinterpret_cast<MiniZinc::SetLit*>(setlit_ptr);
    if (!setlit || index >= setlit->v().size()) {
        return nullptr; // Or handle error appropriately
    }
    return setlit->v()[index];
}

} // extern "C"
