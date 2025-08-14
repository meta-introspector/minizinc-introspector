#include "minizinc_c_wrapper.h"
#include <minizinc/model.hh>

extern "C" {

const char* minizinc_get_version_string() {
    return "2.9.4-introspector";
}

} // extern "C"
