use crate::types::{MiniZincArrayLit, MiniZincExpression};
use crate::ffi_bindings::{arraylit_get_size, arraylit_get_element_at_index};

impl MiniZincArrayLit {
    pub fn size(&self) -> u32 {
        unsafe { arraylit_get_size(self.0) }
    }

    pub fn get_element_at_index(&self, index: u32) -> Option<MiniZincExpression> {
        let expr_ptr = unsafe { arraylit_get_element_at_index(self.0, index) };
        if expr_ptr.is_null() {
            None
        } else {
            Some(MiniZincExpression(expr_ptr))
        }
    }
}
