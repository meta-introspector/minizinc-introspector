#include "minizinc_opaque_types.h"
#include "minizinc_env_wrapper.h"
#include <minizinc/file_utils.hh>
#include <iostream>

extern "C" {

const char* minizinc_get_executable_path() {
    static std::string path = MiniZinc::FileUtils::progpath();
    return path.c_str();
}

} // extern "C"
