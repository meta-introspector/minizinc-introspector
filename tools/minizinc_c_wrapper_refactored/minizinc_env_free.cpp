#include "minizinc_opaque_types.h"
#include "minizinc_env_wrapper.h"
#include <iostream>

extern "C" {

void minizinc_env_free(MiniZinc::Flattener* env) {
    std::cerr << "DEBUG: minizinc_env_free - Freeing Wrapper at: " << env << std::endl; std::cerr.flush();
    delete reinterpret_cast<MiniZinc::MiniZincEnvWrapper*>(env);
}

} // extern "C"