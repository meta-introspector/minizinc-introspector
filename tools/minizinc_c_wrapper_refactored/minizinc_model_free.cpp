#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>

extern "C" {

void minizinc_model_free(MiniZincModel* model) {
    delete reinterpret_cast<MiniZinc::Model*>(model);
}

} // extern "C"