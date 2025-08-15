use crate::types::MiniZincEnvironment;
use crate::ffi_bindings::minizinc_solver_instance_get_solution_value_int;
use std::ffi::CString;

impl MiniZincEnvironment {
    pub fn get_solution_value_int(&self, si_ptr: *mut std::os::raw::c_void, var_name: &str) -> i32 {
        let c_var_name = CString::new(var_name).expect("Failed to convert var_name to CString");
        unsafe {
            minizinc_solver_instance_get_solution_value_int(si_ptr, c_var_name.as_ptr())
        }
    }
}