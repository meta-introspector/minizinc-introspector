use crate::types::MiniZincEnvironment;
use crate::ffi_bindings::minizinc_solver_instance_print_solution;

impl MiniZincEnvironment {
    pub fn solver_instance_print_solution(&self, si_ptr: *mut std::os::raw::c_void) {
        unsafe {
            minizinc_solver_instance_print_solution(si_ptr)
        }
    }
}