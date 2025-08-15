#include "minizinc_opaque_types.h"
#include "minizinc_env_wrapper.h"
#include <iostream>
#include <string>

extern "C" {

MiniZinc::Flattener* minizinc_env_new() {
    std::string stdlib_path = "/data/data/com.termux/files/home/storage/github/libminizinc/install/share/minizinc";
    std::cerr << "DEBUG: minizinc_env_new - Using stdlib_path: " << std::endl; std::cerr.flush();
    
    // Pass true for verbose mode
    MiniZinc::MiniZincEnvWrapper* new_wrapper = new MiniZinc::MiniZincEnvWrapper(std::cout, std::cerr, stdlib_path, true);
    std::cerr << "DEBUG: minizinc_env_new - Created Wrapper at: " << new_wrapper << std::endl; std::cerr.flush();

    return reinterpret_cast<MiniZinc::Flattener*>(new_wrapper);
}

} // extern "C"
