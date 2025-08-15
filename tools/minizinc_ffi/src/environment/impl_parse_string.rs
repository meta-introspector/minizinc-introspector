use std::ffi::{CStr, CString};
use crate::types::MiniZincModel;
use super::minizinc_environment_struct::MiniZincEnvironment;

impl MiniZincEnvironment {
    pub fn parse_string(&self, model_code: &str) -> Result<MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        let model_ptr = unsafe {
            crate::ffi_bindings::minizinc_parse_string_only(
                self.0,
                model_cstr.as_ptr(),
            )
        };
        if model_ptr.is_null() {
            Err("Failed to parse MiniZinc model from string".to_string())
        } else {
            Ok(MiniZincModel(model_ptr))
        }
    }
}