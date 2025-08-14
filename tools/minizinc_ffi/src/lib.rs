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

    // New functions for MiniZincModel inspection
    fn model_get_filename(model_ptr: *mut MiniZincModel) -> *const c_char;
    fn model_get_filepath(model_ptr: *mut MiniZincModel) -> *const c_char;
    fn model_get_num_items(model_ptr: *mut MiniZincModel) -> u32;
    fn model_get_item_at_index(model_ptr: *mut MiniZincModel, index: u32) -> *mut Item;
}

// Safe Rust wrappers for FFI functions
pub struct MiniZincEnvironment(*mut Flattener);
pub struct MiniZincModel(*mut MiniZincModel);
pub struct MiniZincItem(*mut Item);

impl MiniZincEnvironment {
    pub fn new() -> Result<Self, String> {
        let env_ptr = unsafe { minizinc_env_new() };
        if env_ptr.is_null() {
            Err("Failed to create MiniZinc environment".to_string())
        } else {
            Ok(MiniZincEnvironment(env_ptr))
        }
    }

    pub fn parse_model(&self, model_code: &str, filename: &str) -> Result<MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let model_ptr = unsafe {
            minizinc_parse_model(
                self.0,
                model_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if model_ptr.is_null() {
            Err("Failed to parse MiniZinc model".to_string())
        } else {
            Ok(MiniZincModel(model_ptr))
        }
    }

    pub fn parse_data(&self, model: &MiniZincModel, data_code: &str, filename: &str) -> Result<(), String> {
        let data_cstr = CString::new(data_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let result = unsafe {
            minizinc_parse_data_from_string(
                self.0, // Pass Flattener*
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
}

impl Drop for MiniZincEnvironment {
    fn drop(&mut self) {
        unsafe { minizinc_env_free(self.0) };
    }
}

impl Drop for MiniZincModel {
    fn drop(&mut self) {
        unsafe { minizinc_model_free(self.0) };
    }
}

impl MiniZincModel {
    pub fn filename(&self) -> String {
        let c_str = unsafe { model_get_filename(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn filepath(&self) -> String {
        let c_str = unsafe { model_get_filepath(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn num_items(&self) -> u32 {
        unsafe { model_get_num_items(self.0) }
    }

    pub fn get_item_at_index(&self, index: u32) -> Option<MiniZincItem> {
        let item_ptr = unsafe { model_get_item_at_index(self.0, index) };
        if item_ptr.is_null() {
            None
        } else {
            Some(MiniZincItem(item_ptr))
        }
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
        let model_code = "int: x; solve satisfy;";
        let filename = "test_model.mzn";
        let model = env.parse_model(model_code, filename);
        assert!(model.is_ok());
        let model = model.unwrap();
        // Model is automatically freed by Drop trait

        println!("Parsed Model Filename: {}", model.filename());
        println!("Parsed Model Filepath: {}", model.filepath());
        println!("Parsed Model Num Items: {}", model.num_items());

        assert!(!model.filename().is_empty());
        assert!(!model.filepath().is_empty());
        assert!(model.num_items() > 0);
    }

    #[test]
    fn test_parse_data_from_string() {
        let env = MiniZincEnvironment::new().unwrap();
        // Model with x as a parameter (to be defined by data)
        let model_code = "int: x; solve satisfy;";
        let model_filename = "test_model_for_data.mzn";
        let model = env.parse_model(model_code, model_filename);
        assert!(model.is_ok()); // Ensure model parsing itself is successful
        let model = model.unwrap();

        let data_code = "x = 10;";
        let data_filename = "test_data.dzn";
        let result = env.parse_data(&model, data_code, data_filename); // Pass the model reference
        assert!(result.is_ok());

        // Model is automatically freed by Drop trait
    }
}