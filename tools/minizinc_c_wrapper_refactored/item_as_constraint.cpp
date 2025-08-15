#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::ConstraintI* item_as_constraint(MiniZincItem* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    if (!item || !item->isa<MiniZinc::ConstraintI>()) {
        return nullptr;
    }
    return item->cast<MiniZinc::ConstraintI>();
}

} // extern "C"
