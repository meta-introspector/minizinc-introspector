use crate::item_id::ItemId;
use crate::types::{MiniZincItem, MiniZincVarDecl, MiniZincAssignItem, MiniZincConstraintItem, MiniZincIncludeItem, MiniZincFunctionItem};
use crate::ffi_bindings::{item_get_id, item_is_vardecl, item_as_vardecl, item_is_assign, item_as_assign, item_is_constraint, item_as_constraint, item_is_include, item_as_include, item_is_function, item_as_function};

impl MiniZincItem {
    pub fn item_id(&self) -> ItemId {
        let id = unsafe { item_get_id(self.0) };
        id.into()
    }

    pub fn is_vardecl(&self) -> bool {
        unsafe { item_is_vardecl(self.0) }
    }

    pub fn as_vardecl(&self) -> Option<MiniZincVarDecl> {
        let vardecl_ptr = unsafe { item_as_vardecl(self.0) };
        if vardecl_ptr.is_null() {
            None
        } else {
            Some(MiniZincVarDecl(vardecl_ptr))
        }
    }

    pub fn is_assign(&self) -> bool {
        unsafe { item_is_assign(self.0) }
    }

    pub fn as_assign(&self) -> Option<MiniZincAssignItem> {
        let assign_ptr = unsafe { item_as_assign(self.0) };
        if assign_ptr.is_null() {
            None
        } else {
            Some(MiniZincAssignItem(assign_ptr))
        }
    }

    pub fn is_constraint(&self) -> bool {
        unsafe { item_is_constraint(self.0) }
    }

    pub fn as_constraint(&self) -> Option<MiniZincConstraintItem> {
        let constraint_ptr = unsafe { item_as_constraint(self.0) };
        if constraint_ptr.is_null() {
            None
        } else {
            Some(MiniZincConstraintItem(constraint_ptr))
        }
    }

    pub fn is_include(&self) -> bool {
        unsafe { item_is_include(self.0) }
    }

    pub fn as_include(&self) -> Option<MiniZincIncludeItem> {
        let include_ptr = unsafe { item_as_include(self.0) };
        if include_ptr.is_null() {
            None
        } else {
            Some(MiniZincIncludeItem(include_ptr))
        }
    }

    pub fn is_function(&self) -> bool {
        unsafe { item_is_function(self.0) }
    }

    pub fn as_function(&self) -> Option<MiniZincFunctionItem> {
        let function_ptr = unsafe { item_as_function(self.0) };
        if function_ptr.is_null() {
            None
        } else {
            Some(MiniZincFunctionItem(function_ptr))
        }
    }
}