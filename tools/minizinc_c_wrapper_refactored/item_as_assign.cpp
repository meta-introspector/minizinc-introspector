#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::AssignI* item_as_assign(MiniZincItem* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    if (!item || !item->isa<MiniZinc::AssignI>()) {
        return nullptr;
    }
    return item->cast<MiniZinc::AssignI>();
}

} // extern "C"
