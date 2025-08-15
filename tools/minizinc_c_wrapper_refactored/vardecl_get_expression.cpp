#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

MiniZinc::Expression* vardecl_get_expression(MiniZinc::VarDeclI* vardecl_ptr) {
    MiniZinc::VarDeclI* vardecl = reinterpret_cast<MiniZinc::VarDeclI*>(vardecl_ptr);
    return vardecl->e()->e();
}

} // extern "C"