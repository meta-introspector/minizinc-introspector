#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::TypeInst* vardecl_get_type_inst(MiniZinc::VarDeclI* vardecl_ptr) {
    MiniZinc::VarDeclI* vardecl = reinterpret_cast<MiniZinc::VarDeclI*>(vardecl_ptr);
    return vardecl->e()->ti();
}

} // extern "C"
