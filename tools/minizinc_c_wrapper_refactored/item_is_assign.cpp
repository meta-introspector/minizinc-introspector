#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool item_is_assign(Item* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    if (!item) {
        return false;
    }
    return item->isa<MiniZinc::AssignI>();
}

} // extern "C"
