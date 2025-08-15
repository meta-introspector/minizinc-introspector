use std::ffi::CStr;
use super::minizinc_environment_struct::MiniZincEnvironment;

impl MiniZincEnvironment {
    pub fn get_executable_path(&self) -> String {
        let path_cstr = unsafe { crate::ffi_bindings::minizinc_get_executable_path() };
        unsafe { CStr::from_ptr(path_cstr).to_str().unwrap().to_string() }
    }
}