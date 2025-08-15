#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>

extern "C" {

MiniZincItem* model_get_item_at_index(MiniZincModel* model_ptr, uint32_t index) {
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    if (index < model->size()) {
        MiniZinc::Item* item_ptr = model->operator[](index);
        return reinterpret_cast<MiniZincItem*>(item_ptr);
    }
    return nullptr;
}

} // extern "C"