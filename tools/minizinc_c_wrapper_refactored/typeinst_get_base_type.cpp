#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

int typeinst_get_base_type(MiniZinc::TypeInst* typeinst_ptr) {
    MiniZinc::TypeInst* type_inst = reinterpret_cast<MiniZinc::TypeInst*>(typeinst_ptr);
    return static_cast<int>(type_inst->type().bt());
}

} // extern "C"