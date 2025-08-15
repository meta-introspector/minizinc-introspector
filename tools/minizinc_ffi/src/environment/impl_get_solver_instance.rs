use crate::types::MiniZincEnvironment;
use crate::ffi_bindings::minizinc_solver_get_solver_instance;

impl MiniZincEnvironment {
    pub fn get_solver_instance(&self) -> *mut std::os::raw::c_void {
        unsafe {
            minizinc_solver_get_solver_instance(self.0)
        }
    }
}