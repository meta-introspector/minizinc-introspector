use std::ffi::{CStr, CString};
//use std::os::raw::c_char;
use crate::types::{MznSolver, MiniZincModel};
use crate::ffi_bindings::{minizinc_env_new, minizinc_env_free, minizinc_parse_model_with_flags, minizinc_parse_data_from_string, minizinc_get_version_string, minizinc_get_mznlib_dir, minizinc_get_executable_path, minizinc_parse_string_only};

// Safe Rust wrappers for FFI functions
pub struct MiniZincEnvironment(pub *mut MznSolver);

impl MiniZincEnvironment {
    pub fn new() -> Result<Self, String> {
        let env_ptr = unsafe { minizinc_env_new() };
        if env_ptr.is_null() {
            Err("Failed to create MiniZinc environment".to_string())
        } else {
            Ok(MiniZincEnvironment(env_ptr))
        }
    }

    // Deprecated: Use parse_string instead for string-only parsing
    // pub fn parse_model(&self, model_code: &str, filename: &str) -> Result<MiniZincModel, String> {
    //     let model_cstr = CString::new(model_code).expect("CString::new failed");
        
    //     // If filename is empty, use a dummy filename to avoid MiniZinc's "Cannot open file ''" error
    //     let actual_filename = if filename.is_empty() {
    //         "dummy_model.mzn" // Use a non-empty dummy filename
    //     } else {
    //         filename
    //     };
    //     let filename_cstr = CString::new(actual_filename).expect("CString::new failed");

    //     let model_ptr = unsafe {
    //         minizinc_parse_model_with_flags(
    //             self.0,
    //             model_cstr.as_ptr(),
    //             filename_cstr.as_ptr(),
    //             true, // is_model_string is true when parsing from a string
    //         )
    //     };
    //     if model_ptr.is_null() {
    //         Err("Failed to parse MiniZinc model".to_string())
    //     } else {
    //         Ok(MiniZincModel(model_ptr))
    //     }
    // }

    pub fn parse_string(&self, model_code: &str) -> Result<MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        let model_ptr = unsafe {
            minizinc_parse_string_only(
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

    pub fn parse_data(&self, model: &MiniZincModel, data_code: &str, filename: &str) -> Result<(), String> {
        let data_cstr = CString::new(data_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let result = unsafe {
            minizinc_parse_data_from_string(
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

    pub fn get_version_string(&self) -> String {
        let version_cstr = unsafe { minizinc_get_version_string() };
        unsafe { CStr::from_ptr(version_cstr).to_str().unwrap().to_string() }
    }

    pub fn get_mznlib_dir(&self) -> String {
        let mznlib_dir_cstr = unsafe { minizinc_get_mznlib_dir(self.0) };
        unsafe { CStr::from_ptr(mznlib_dir_cstr).to_str().unwrap().to_string() }
    }

    pub fn get_executable_path() -> String {
        let path_cstr = unsafe { minizinc_get_executable_path() };
        unsafe { CStr::from_ptr(path_cstr).to_str().unwrap().to_string() }
    }
}

impl Drop for MiniZincEnvironment {
    fn drop(&mut self) {
        unsafe { minizinc_env_free(self.0) };
    }
}
