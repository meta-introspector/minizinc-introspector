#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::Expression* arraylit_get_element_at_index(MiniZinc::ArrayLit* arraylit_ptr, unsigned int index) {
    MiniZinc::ArrayLit* arraylit = reinterpret_cast<MiniZinc::ArrayLit*>(arraylit_ptr);
    if (!arraylit || index >= arraylit->size()) {
        return nullptr; // Or handle error appropriately
    }
    return (*arraylit)[index];
}

} // extern "C"
