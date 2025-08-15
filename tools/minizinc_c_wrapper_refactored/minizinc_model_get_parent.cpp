#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>

extern "C" {

MiniZincModel* minizinc_model_get_parent(MiniZincModel* model_ptr) {
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    if (!model) {
        return nullptr; // Or handle error appropriately
    }
    return reinterpret_cast<MiniZincModel*>(model->parent());
}

} // extern "C"
