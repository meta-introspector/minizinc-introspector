#include "minizinc_opaque_types.h"
#include "minizinc_env_wrapper.h"
#include <iostream>

extern "C" {

const char* minizinc_get_mznlib_dir(MiniZinc::Flattener* env_ptr) {
    MiniZinc::MiniZincEnvWrapper* wrapper = reinterpret_cast<MiniZinc::MiniZincEnvWrapper*>(env_ptr);
    return wrapper->getSolverConfigs().mznlibDir().c_str();
}

} // extern "C"
