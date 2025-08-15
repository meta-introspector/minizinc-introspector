use std::ffi::{CString, CStr};
use crate::types::MiniZincEnvironment;
use crate::ffi_bindings::{minizinc_solver_run};

impl MiniZincEnvironment {
    pub fn run_model(&self, model_str: &str, args: &[&str]) -> i32 {
        let c_model_str = CString::new(model_str).expect("Failed to convert model string to CString");

        let c_args: Vec<CString> = args.iter().map(|&s| CString::new(s).expect("Failed to convert arg to CString")).collect();
        let c_args_ptrs: Vec<*const i8> = c_args.iter().map(|s| s.as_ptr()).collect();

        unsafe {
            minizinc_solver_run(
                self.0,
                c_model_str.as_ptr(),
                c_args_ptrs.as_ptr(),
                c_args_ptrs.len() as i32,
            )
        }
    }
}