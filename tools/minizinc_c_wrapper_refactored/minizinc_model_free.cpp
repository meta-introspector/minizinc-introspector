#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>

extern "C" {

void minizinc_model_free(MiniZincModel* model) {
    std::cerr << "[minizinc_model_free] Called with model pointer: " << model << std::endl; std::cerr.flush();
    if (model == nullptr) {
        std::cerr << "[minizinc_model_free] WARNING: Attempted to free a nullptr model." << std::endl; std::cerr.flush();
        return;
    }
    MiniZinc::Model* mzn_model = reinterpret_cast<MiniZinc::Model*>(model);
    std::cerr << "[minizinc_model_free] Deleting MiniZinc::Model* at: " << mzn_model << std::endl; std::cerr.flush();
    delete mzn_model;
    std::cerr << "[minizinc_model_free] Model deleted." << std::endl; std::cerr.flush();
}

} // extern "C"