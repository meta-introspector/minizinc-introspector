use crate::environment::MiniZincEnvironment;
use crate::ffi_bindings::minizinc_solver_instance_next;

impl MiniZincEnvironment {
    pub fn solver_instance_next(&self, solver_instance_ptr: *mut std::os::raw::c_void) -> i32 {
        unsafe {
            minizinc_solver_instance_next(solver_instance_ptr)
        }
    }
}