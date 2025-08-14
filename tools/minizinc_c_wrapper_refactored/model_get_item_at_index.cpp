#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>

extern "C" {

Item* model_get_item_at_index(MiniZincModel* model_ptr, uint32_t index) {
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    if (index < model->size()) {
        return model->operator[](index);
    }
    return nullptr;
}

} // extern "C"