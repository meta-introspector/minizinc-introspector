#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>
#include <cstring> // For strdup

extern "C" {

char* model_get_filepath(MiniZincModel* model_ptr) {
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    return strdup(model->filepath().c_str());
}

} // extern "C"