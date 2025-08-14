#include "minizinc_c_wrapper.h"
#include <minizinc/model.hh>

extern "C" {

uint32_t model_get_num_items(MiniZincModel* model_ptr) {
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    return model->size();
}

} // extern "C"
