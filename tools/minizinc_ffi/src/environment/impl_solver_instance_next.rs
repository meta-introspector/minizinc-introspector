use crate::environment::MiniZincEnvironment;
use crate::ffi_bindings::minizinc_solver_instance_next;

impl MiniZincEnvironment {
    /// # Safety
    ///
    /// This function is unsafe because it dereferences a raw C pointer (`solver_instance_ptr`).
    /// The caller must ensure that `solver_instance_ptr` is a valid, non-null pointer
    /// to a `minizinc_solver_instance` object obtained from the MiniZinc C API.
    pub unsafe fn solver_instance_next(&self, solver_instance_ptr: *mut std::os::raw::c_void) -> i32 {
        unsafe {
            minizinc_solver_instance_next(solver_instance_ptr)
        }
    }
}