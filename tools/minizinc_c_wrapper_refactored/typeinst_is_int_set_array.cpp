#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>

extern "C" {

bool typeinst_is_int_set_array(MiniZinc::TypeInst* typeinst_ptr) {
    MiniZinc::TypeInst* typeinst = reinterpret_cast<MiniZinc::TypeInst*>(typeinst_ptr);
    if (!typeinst) {
        return false;
    }
    return typeinst->type().isIntSetArray();
}

} // extern "C"
