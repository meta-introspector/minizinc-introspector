#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::FunctionI* item_as_function(MiniZincItem* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    if (!item || !item->isa<MiniZinc::FunctionI>()) {
        return nullptr;
    }
    return item->cast<MiniZinc::FunctionI>();
}

} // extern "C"
