#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

int vardecl_get_payload(MiniZinc::VarDeclI* vardecl_ptr) {
    MiniZinc::VarDecl* vardecl = reinterpret_cast<MiniZinc::VarDecl*>(vardecl_ptr->e());
    if (!vardecl) {
        return 0; // Or handle error appropriately
    }
    return vardecl->payload();
}

} // extern "C"
