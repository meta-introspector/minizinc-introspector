use crate::environment::MiniZincEnvironment;
use crate::ffi_bindings::minizinc_solver_get_solution_value_int;
use std::ffi::CString;

impl MiniZincEnvironment {
    pub fn get_solution_value_int(&self, var_name: &str) -> i32 {
        let c_var_name = CString::new(var_name).expect("Failed to convert var_name to CString");
        unsafe {
            minizinc_solver_get_solution_value_int(self.0, c_var_name.as_ptr())
        }
    }
}