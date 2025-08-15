#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>

extern "C" {

SolveI* minizinc_model_get_solve_item(MiniZincModel* model_ptr) {
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    if (!model) {
        return nullptr; // Or handle error appropriately
    }
    return model->solveItem();
}

} // extern "C"
