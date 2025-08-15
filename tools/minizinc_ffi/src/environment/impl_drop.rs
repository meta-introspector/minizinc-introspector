use super::minizinc_environment_struct::MiniZincEnvironment;

impl Drop for MiniZincEnvironment {
    fn drop(&mut self) {
        unsafe { crate::ffi_bindings::minizinc_env_free(self.0) };
    }
}