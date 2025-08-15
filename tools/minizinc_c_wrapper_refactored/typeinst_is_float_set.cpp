#include "minizinc_opaque_types.h"
#include <minizinc/type.hh>

extern "C" {

bool typeinst_is_float_set(MiniZinc::TypeInst* typeinst_ptr) {
    MiniZinc::TypeInst* typeinst = reinterpret_cast<MiniZinc::TypeInst*>(typeinst_ptr);
    if (!typeinst) {
        return false;
    }
    return typeinst->type().isFloatSet();
}

} // extern "C"
