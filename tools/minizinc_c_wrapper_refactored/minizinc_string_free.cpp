#include "minizinc_opaque_types.h"
#include <cstdlib> // For free

extern "C" {

void minizinc_string_free(char* s) {
    free(s);
}

} // extern "C"