use std::ffi::CStr;
use crate::types::MiniZincId;
use crate::ffi_bindings;

impl MiniZincId {
    pub fn value(&self) -> String {
        let c_str = unsafe { ffi_bindings::id_get_value(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }
}