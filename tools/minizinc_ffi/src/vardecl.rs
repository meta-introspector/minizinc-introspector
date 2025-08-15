use std::ffi::CStr;
use crate::types::{MiniZincExpression, MiniZincTypeInst, MiniZincVarDecl};
use crate::ffi_bindings::{vardecl_get_id, vardecl_get_type_inst, vardecl_get_expression, vardecl_is_toplevel, vardecl_is_introduced, vardecl_is_evaluated, vardecl_get_payload, vardecl_is_type_alias};

impl MiniZincVarDecl {
    pub fn id(&self) -> String {
        let c_str = unsafe { vardecl_get_id(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn type_inst(&self) -> MiniZincTypeInst {
        let type_inst_ptr = unsafe { vardecl_get_type_inst(self.0) };
        MiniZincTypeInst(type_inst_ptr)
    }

    pub fn expression(&self) -> Option<MiniZincExpression> {
        let expr_ptr = unsafe { vardecl_get_expression(self.0) };
        if expr_ptr.is_null() {
            None
        } else {
            Some(MiniZincExpression(expr_ptr))
        }
    }

    pub fn is_toplevel(&self) -> bool {
        unsafe { vardecl_is_toplevel(self.0) }
    }

    pub fn is_introduced(&self) -> bool {
        unsafe { vardecl_is_introduced(self.0) }
    }

    pub fn is_evaluated(&self) -> bool {
        unsafe { vardecl_is_evaluated(self.0) }
    }

    pub fn payload(&self) -> i32 {
        unsafe { vardecl_get_payload(self.0) }
    }

    pub fn is_type_alias(&self) -> bool {
        unsafe { vardecl_is_type_alias(self.0) }
    }
}
