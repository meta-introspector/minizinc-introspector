use crate::types::MiniZincBoolLit;
use crate::ffi_bindings::boollit_get_value;

impl MiniZincBoolLit {
    pub fn value(&self) -> bool {
        unsafe { boollit_get_value(self.0) }
    }
}
