#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh>

extern "C" {

void minizinc_env_free(MiniZincEnvWrapper* wrapper_ptr) {
    MiniZincEnvWrapper* wrapper = reinterpret_cast<MiniZincEnvWrapper*>(wrapper_ptr);
    if (wrapper) {
        if (wrapper->solver) {
            delete wrapper->solver;
        }
        if (wrapper->timer) {
            delete wrapper->timer;
        }
        delete wrapper;
    }
}

} // extern "C"