use std::ffi::CString;
use crate::types::MiniZincModel;
use super::minizinc_environment_struct::MiniZincEnvironment;

impl MiniZincEnvironment {
    pub fn parse_data(&self, model: &MiniZincModel, data_code: &str, filename: &str) -> Result<(), String> {
        let data_cstr = CString::new(data_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let result = unsafe {
            crate::ffi_bindings::minizinc_parse_data_from_string(
                self.0,
                model.0,
                data_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if result != 0 {
            Err("Failed to parse MiniZinc data".to_string())
        } else {
            Ok(())
        }
    }
}