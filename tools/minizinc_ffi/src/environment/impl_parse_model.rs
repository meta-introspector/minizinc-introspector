use std::ffi::{CStr, CString};
use crate::types::MiniZincModel;
use super::minizinc_environment_struct::MiniZincEnvironment;

impl MiniZincEnvironment {
    pub fn parse_model(&self, model_code: &str, filename: &str) -> Result<MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        
        // If filename is empty, use a dummy filename to avoid MiniZinc's "Cannot open file ''" error
        let actual_filename = if filename.is_empty() {
            "dummy_model.mzn" // Use a non-empty dummy filename
        } else {
            filename
        };
        let filename_cstr = CString::new(actual_filename).expect("CString::new failed");

        let model_ptr = unsafe {
            crate::ffi_bindings::minizinc_parse_model_with_flags(
                self.0,
                model_cstr.as_ptr(),
                filename_cstr.as_ptr(),
                true, // is_model_string is true when parsing from a string
            )
        };
        if model_ptr.is_null() {
            Err("Failed to parse MiniZinc model".to_string())
        } else {
            Ok(MiniZincModel(model_ptr))
        }
    }
}