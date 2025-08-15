use crate::types::MiniZincBoolLit;
use crate::ffi_bindings;

impl MiniZincBoolLit {
    pub fn value(&self) -> bool {
        unsafe { ffi_bindings::boollit_get_value(self.0) }
    }
}