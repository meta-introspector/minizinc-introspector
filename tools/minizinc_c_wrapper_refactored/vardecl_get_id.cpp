#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

const char* vardecl_get_id(MiniZinc::VarDeclI* vardecl_ptr) {
    MiniZinc::VarDeclI* vardecl = reinterpret_cast<MiniZinc::VarDeclI*>(vardecl_ptr);
    return vardecl->e()->id()->v().c_str();
}

} // extern "C"