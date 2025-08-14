use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Opaque types for MiniZincModel and Flattener
type MiniZincModel = std::os::raw::c_void;
type Flattener = std::os::raw::c_void; // Changed from MiniZincEnv

unsafe extern "C" {
    fn minizinc_env_new() -> *mut Flattener; // Updated return type
    fn minizinc_env_free(env: *mut Flattener); // Updated parameter type
    fn minizinc_parse_model( // Updated function name
        env: *mut Flattener, // Updated parameter type
        model_str: *const c_char,
        filename: *const c_char,
    ) -> *mut MiniZincModel;
    fn minizinc_parse_data_from_string(
        env: *mut Flattener, // Updated parameter type
        model: *mut MiniZincModel,
        data_str: *const c_char,
        filename: *const c_char,
    ) -> std::os::raw::c_int;
    fn minizinc_model_free(model: *mut MiniZincModel);
    fn minizinc_get_version_string() -> *const c_char;
}

// Safe Rust wrappers for FFI functions
pub struct MiniZincEnvironment(*mut Flattener); // Updated to hold Flattener

impl MiniZincEnvironment {
    pub fn new() -> Result<Self, String> {
        let env_ptr = unsafe { minizinc_env_new() };
        if env_ptr.is_null() {
            Err("Failed to create MiniZinc environment".to_string())
        } else {
            Ok(MiniZincEnvironment(env_ptr))
        }
    }

    pub fn parse_model(&self, model_code: &str, filename: &str) -> Result<*mut MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let model_ptr = unsafe {
            minizinc_parse_model( // Updated function call
                self.0,
                model_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if model_ptr.is_null() {
            Err("Failed to parse MiniZinc model".to_string())
        } else {
            Ok(model_ptr)
        }
    }

    pub fn parse_data(&self, model: *mut MiniZincModel, data_code: &str, filename: &str) -> Result<(), String> {
        let data_cstr = CString::new(data_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let result = unsafe {
            minizinc_parse_data_from_string(
                self.0, // Pass Flattener*
                model,
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
}

impl Drop for MiniZincEnvironment {
    fn drop(&mut self) {
        unsafe { minizinc_env_free(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_string() {
        let env = MiniZincEnvironment::new().unwrap();
        let version = env.get_version_string();
        println!("MiniZinc Version: {}", version);
        assert_eq!(version, "2.9.4-introspector");
    }

    #[test]
    fn test_env_creation_and_free() {
        let env = MiniZincEnvironment::new();
        assert!(env.is_ok());
        // Drop will be called automatically when env goes out of scope
    }

    #[test]
    fn test_parse_model_from_string() {
        let env = MiniZincEnvironment::new().unwrap();
        // Model with x defined
        let model_code = "int: x = 1; solve satisfy;";
        let filename = "test_model.mzn";
        let model_ptr = env.parse_model(model_code, filename);
        assert!(model_ptr.is_ok());
        let model = model_ptr.unwrap();
        assert!(!model.is_null());
        unsafe { minizinc_model_free(model) }; // Manually free the model
    }

    #[test]
    fn test_parse_data_from_string() {
        let env = MiniZincEnvironment::new().unwrap();
        // Model with x as a parameter (to be defined by data)
        let model_code = "int: x; solve satisfy;";
        let model_filename = "test_model_for_data.mzn";
        let model_ptr = env.parse_model(model_code, model_filename);
        assert!(model_ptr.is_ok()); // Ensure model parsing itself is successful
        let model = model_ptr.unwrap();
        assert!(!model.is_null());

        let data_code = "x = 10;";
        let data_filename = "test_data.dzn";
        let result = env.parse_data(model, data_code, data_filename); // Pass the model pointer
        assert!(result.is_ok());

        unsafe { minizinc_model_free(model) }; // Manually free the model
    }
}