use std::ffi::CStr;
use super::minizinc_environment_struct::MiniZincEnvironment;

impl MiniZincEnvironment {
    pub fn get_version_string(&self) -> String {
        let version_cstr = unsafe { crate::ffi_bindings::minizinc_get_version_string() };
        unsafe { CStr::from_ptr(version_cstr).to_str().unwrap().to_string() }
    }
}