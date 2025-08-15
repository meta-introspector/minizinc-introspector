use crate::types::MiniZincEnvironment;
use crate::ffi_bindings::minizinc_solver_instance_next;

impl MiniZincEnvironment {
    pub fn solver_instance_next(&self, si_ptr: *mut std::os::raw::c_void) -> i32 {
        unsafe {
            minizinc_solver_instance_next(si_ptr)
        }
    }
}