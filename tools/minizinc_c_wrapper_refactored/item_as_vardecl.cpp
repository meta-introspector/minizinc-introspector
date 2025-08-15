#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::VarDeclI* item_as_vardecl(MiniZincItem* item_ptr) {
    MiniZinc::Item* item = reinterpret_cast<MiniZinc::Item*>(item_ptr);
    return item->cast<MiniZinc::VarDeclI>();
}

} // extern "C"