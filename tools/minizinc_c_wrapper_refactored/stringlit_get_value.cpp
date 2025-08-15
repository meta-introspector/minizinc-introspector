#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>
#include <string>

extern "C" {

const char* stringlit_get_value(MiniZinc::StringLit* stringlit_ptr) {
    MiniZinc::StringLit* stringlit = reinterpret_cast<MiniZinc::StringLit*>(stringlit_ptr);
    if (!stringlit) {
        return nullptr; // Or handle error appropriately
    }
    // MiniZinc::StringLit::v() returns an ASTString, which can be converted to std::string
    // We need to make a copy that outlives this function call.
    return strdup(stringlit->v().c_str());
}

} // extern "C"
