use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Opaque types for MiniZincModel and MiniZincEnv
type MiniZincModel = std::os::raw::c_void;
type MiniZincEnv = std::os::raw::c_void;

unsafe extern "C" {
    fn minizinc_env_new() -> *mut MiniZincEnv;
    fn minizinc_env_free(env: *mut MiniZincEnv);
    fn minizinc_parse_model_from_string(
        env: *mut MiniZincEnv,
        model_str: *const c_char,
        filename: *const c_char,
    ) -> *mut MiniZincModel;
    fn minizinc_parse_data_from_string(
        env: *mut MiniZincEnv,
        model: *mut MiniZincModel,
        data_str: *const c_char,
        filename: *const c_char,
    ) -> std::os::raw::c_int;
    fn minizinc_model_free(model: *mut MiniZincModel);
    fn minizinc_get_version_string() -> *const c_char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_string() {
        let version_cstr = unsafe { minizinc_get_version_string() };
        let version = unsafe { CStr::from_ptr(version_cstr).to_str().unwrap() };
        println!("MiniZinc Version: {}", version);
        assert_eq!(version, "2.9.4-introspector");
    }
}