#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::IncludeI* item_as_include(Item* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    if (!item || !item->isa<MiniZinc::IncludeI>()) {
        return nullptr;
    }
    return item->cast<MiniZinc::IncludeI>();
}

} // extern "C"
