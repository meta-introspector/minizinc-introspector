#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool item_is_vardecl(MiniZincItem* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    return item->isa<MiniZinc::VarDeclI>();
}

} // extern "C"
