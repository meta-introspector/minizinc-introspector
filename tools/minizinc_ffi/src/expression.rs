use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::types::{MiniZincFloatLit, MiniZincSetLit, MiniZincBoolLit, MiniZincStringLit, MiniZincId, MiniZincAnonVar, MiniZincArrayLit};
use crate::expression_id::MiniZincExpressionId;
use crate::ffi_bindings::{expression_get_id, expression_is_intlit, expression_is_floatlit, expression_as_floatlit, floatlit_get_value, expression_is_setlit, expression_as_setlit, expression_is_boollit, expression_as_boollit, expression_is_stringlit, expression_as_stringlit, expression_is_id, expression_as_id, expression_is_anon_var, expression_as_anon_var, expression_is_arraylit, expression_as_arraylit};

#[derive(Debug)]
pub struct MiniZincExpression(pub *mut std::os::raw::c_void);

impl MiniZincExpression {
    pub fn expression_id(&self) -> MiniZincExpressionId {
        let id = unsafe { expression_get_id(self.0) };
        id.into()
    }

    pub fn is_intlit(&self) -> bool {
        unsafe { expression_is_intlit(self.0) }
    }

    pub fn is_floatlit(&self) -> bool {
        unsafe { expression_is_floatlit(self.0) }
    }

    pub fn as_floatlit(&self) -> Option<MiniZincFloatLit> {
        let floatlit_ptr = unsafe { expression_as_floatlit(self.0) };
        if floatlit_ptr.is_null() {
            None
        } else {
            Some(MiniZincFloatLit(floatlit_ptr))
        }
    }

    pub fn is_setlit(&self) -> bool {
        unsafe { expression_is_setlit(self.0) }
    }

    pub fn as_setlit(&self) -> Option<MiniZincSetLit> {
        let setlit_ptr = unsafe { expression_as_setlit(self.0) };
        if setlit_ptr.is_null() {
            None
        } else {
            Some(MiniZincSetLit(setlit_ptr))
        }
    }

    pub fn is_boollit(&self) -> bool {
        unsafe { expression_is_boollit(self.0) }
    }

    pub fn as_boollit(&self) -> Option<MiniZincBoolLit> {
        let boollit_ptr = unsafe { expression_as_boollit(self.0) };
        if boollit_ptr.is_null() {
            None
        } else {
            Some(MiniZincBoolLit(boollit_ptr))
        }
    }

    pub fn is_stringlit(&self) -> bool {
        unsafe { expression_is_stringlit(self.0) }
    }

    pub fn as_stringlit(&self) -> Option<MiniZincStringLit> {
        let stringlit_ptr = unsafe { expression_as_stringlit(self.0) };
        if stringlit_ptr.is_null() {
            None
        } else {
            Some(MiniZincStringLit(stringlit_ptr))
        }
    }

    pub fn is_id(&self) -> bool {
        unsafe { expression_is_id(self.0) }
    }

    pub fn as_id(&self) -> Option<MiniZincId> {
        let id_ptr = unsafe { expression_as_id(self.0) };
        if id_ptr.is_null() {
            None
        } else {
            Some(MiniZincId(id_ptr))
        }
    }

    pub fn is_anon_var(&self) -> bool {
        unsafe { expression_is_anon_var(self.0) }
    }

    pub fn as_anon_var(&self) -> Option<MiniZincAnonVar> {
        let anon_var_ptr = unsafe { expression_as_anon_var(self.0) };
        if anon_var_ptr.is_null() {
            None
        } else {
            Some(MiniZincAnonVar(anon_var_ptr))
        }
    }

    pub fn is_arraylit(&self) -> bool {
        unsafe { expression_is_arraylit(self.0) }
    }

    pub fn as_arraylit(&self) -> Option<MiniZincArrayLit> {
        let arraylit_ptr = unsafe { expression_as_arraylit(self.0) };
        if arraylit_ptr.is_null() {
            None
        } else {
            Some(MiniZincArrayLit(arraylit_ptr))
        }
    }
}
