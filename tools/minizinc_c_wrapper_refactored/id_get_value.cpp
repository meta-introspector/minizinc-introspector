#include "minizinc_opaque_types.h"
#include <minizinc/ast.hh>
#include <string>

extern "C" {

const char* id_get_value(MiniZinc::Id* id_ptr) {
    MiniZinc::Id* id = reinterpret_cast<MiniZinc::Id*>(id_ptr);
    if (!id) {
        return nullptr; // Or handle error appropriately
    }
    // MiniZinc::Id::v() returns an ASTString, which can be converted to std::string
    // We need to make a copy that outlives this function call.
    return strdup(id->v().c_str());
}

} // extern "C"
