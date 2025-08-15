#include "minizinc_opaque_types.h"
#include <minizinc/type.hh>

extern "C" {

bool typeinst_is_par(MiniZinc::TypeInst* typeinst_ptr) {
    MiniZinc::TypeInst* typeinst = reinterpret_cast<MiniZinc::TypeInst*>(typeinst_ptr);
    if (!typeinst) {
        return false;
    }
    return typeinst->type().isPar();
}

} // extern "C"
