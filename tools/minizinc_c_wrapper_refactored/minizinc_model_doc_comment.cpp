#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>
#include <string>

extern "C" {

const char* minizinc_model_get_doc_comment(MiniZincModel* model_ptr) {
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    if (!model) {
        return nullptr; // Or handle error appropriately
    }
    // MiniZinc::Model::docComment() returns a const std::string&
    // We need to make a copy that outlives this function call.
    // This is a common pattern in CFFI where Rust will take ownership and free it.
    return strdup(model->docComment().c_str());
}

} // extern "C"
