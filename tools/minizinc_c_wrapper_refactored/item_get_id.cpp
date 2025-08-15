#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

int item_get_id(MiniZincItem* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    return static_cast<int>(item->iid());
}

} // extern "C"
