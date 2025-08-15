#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/gc.hh> // For MiniZinc::GC

extern "C" {

void minizinc_gc_lock() {
    MiniZinc::GC::lock();
}

} // extern "C"