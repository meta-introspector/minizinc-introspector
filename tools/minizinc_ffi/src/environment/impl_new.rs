use crate::types::MznSolver;
use super::minizinc_environment_struct::MiniZincEnvironment;

impl MiniZincEnvironment {
    pub fn new() -> Result<Self, String> {
        let env_ptr = unsafe { crate::ffi_bindings::minizinc_env_new() };
        if env_ptr.is_null() {
            Err("Failed to create MiniZinc environment".to_string())
        } else {
            Ok(MiniZincEnvironment(env_ptr))
        }
    }
}