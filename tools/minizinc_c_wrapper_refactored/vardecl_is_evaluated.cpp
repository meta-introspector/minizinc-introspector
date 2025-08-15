#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool vardecl_is_evaluated(MiniZinc::VarDeclI* vardecl_ptr) {
    MiniZinc::VarDecl* vardecl = reinterpret_cast<MiniZinc::VarDecl*>(vardecl_ptr->e());
    if (!vardecl) {
        return false;
    }
    return vardecl->evaluated();
}

} // extern "C"
