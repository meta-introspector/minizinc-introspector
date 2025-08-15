use std::ffi::CStr;
use super::minizinc_environment_struct::MiniZincEnvironment;

impl MiniZincEnvironment {
    pub fn get_mznlib_dir(&self) -> String {
        let mznlib_dir_cstr = unsafe { crate::ffi_bindings::minizinc_get_mznlib_dir(self.0) };
        unsafe { CStr::from_ptr(mznlib_dir_cstr).to_str().unwrap().to_string() }
    }
}