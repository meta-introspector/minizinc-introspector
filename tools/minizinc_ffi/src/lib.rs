use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod feature_tests;

// Opaque types for MiniZincModel, Item, SolveItem, and OutputItem
// Opaque types for MiniZincModel, Item, SolveItem, OutputItem, AssignItem, ConstraintItem, IncludeItem, FunctionItem, FloatLit, SetLit, BoolLit, StringLit, and Id
pub struct MiniZincModel(pub *mut std::os::raw::c_void);
pub struct MiniZincItem(pub *mut std::os::raw::c_void);
pub struct MiniZincSolveItem(pub *mut std::os::raw::c_void);
pub struct MiniZincOutputItem(pub *mut std::os::raw::c_void);
pub struct MiniZincAssignItem(pub *mut std::os::raw::c_void);
pub struct MiniZincConstraintItem(pub *mut std::os::raw::c_void);
pub struct MiniZincIncludeItem(pub *mut std::os::raw::c_void);
pub struct MiniZincFunctionItem(pub *mut std::os::raw::c_void);
pub struct MiniZincFloatLit(pub *mut std::os::raw::c_void);
pub struct MiniZincSetLit(pub *mut std::os::raw::c_void);
pub struct MiniZincBoolLit(pub *mut std::os::raw::c_void);
pub struct MiniZincStringLit(pub *mut std::os::raw::c_void);
pub struct MiniZincStringLit(pub *mut std::os::raw::c_void);
pub struct MiniZincId(pub *mut std::os::raw::c_void);

impl MiniZincFloatLit {
    pub fn value(&self) -> f64 {
        unsafe { floatlit_get_value(self.0) }
    }
}

impl MiniZincSetLit {
    pub fn size(&self) -> u32 {
        unsafe { setlit_get_size(self.0) }
    }

    pub fn get_element_at_index(&self, index: u32) -> Option<MiniZincExpression> {
        let expr_ptr = unsafe { setlit_get_element_at_index(self.0, index) };
        if expr_ptr.is_null() {
            None
        } else {
            Some(MiniZincExpression(expr_ptr))
        }
    }
}

impl MiniZincBoolLit {
    pub fn value(&self) -> bool {
        unsafe { boollit_get_value(self.0) }
    }
}

impl MiniZincStringLit {
    pub fn value(&self) -> String {
        let c_str = unsafe { stringlit_get_value(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }
}

#[repr(C)]
pub struct MznSolver { _data: [u8; 0] }

// Import the ItemId enum
mod item_id;
use item_id::ItemId;

// Import the BaseType enum
mod base_type;
use base_type::MiniZincBaseType;

// Import the ExpressionId enum
mod expression_id;
use expression_id::MiniZincExpressionId;

unsafe extern "C" {
    fn minizinc_env_new() -> *mut MznSolver;
    fn minizinc_env_free(env: *mut MznSolver);
    fn minizinc_parse_model(
        env: *mut MznSolver,
        model_str: *const c_char,
        filename: *const c_char,
    ) -> *mut std::os::raw::c_void;
    fn minizinc_parse_data_from_string(
        env: *mut MznSolver,
        model: *mut std::os::raw::c_void,
        data_str: *const c_char,
        filename: *const c_char,
    ) -> std::os::raw::c_int;
    fn minizinc_model_free(model: *mut std::os::raw::c_void);
    fn minizinc_get_version_string() -> *const c_char;

    // New functions for MiniZincModel inspection
    fn model_get_filename(model_ptr: *mut std::os::raw::c_void) -> *const c_char;
    fn model_get_filepath(model_ptr: *mut std::os::raw::c_void) -> *const c_char;
    fn model_get_num_items(model_ptr: *mut std::os::raw::c_void) -> u32;
    fn model_get_item_at_index(model_ptr: *mut std::os::raw::c_void, index: u32) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem inspection
    fn item_get_id(item_ptr: *mut std::os::raw::c_void) -> i32;
    fn item_is_vardecl(item_ptr: *mut std::os::raw::c_void) -> bool;
    fn item_as_vardecl(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem assignment
    fn item_is_assign(item_ptr: *mut std::os::raw::c_void) -> bool;
    fn item_as_assign(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem constraint
    fn item_is_constraint(item_ptr: *mut std::os::raw::c_void) -> bool;
    fn item_as_constraint(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem include
    fn item_is_include(item_ptr: *mut std::os::raw::c_void) -> bool;
    fn item_as_include(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem function
    fn item_is_function(item_ptr: *mut std::os::raw::c_void) -> bool;
    fn item_as_function(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for VarDeclI inspection
    fn vardecl_get_id(vardecl_ptr: *mut std::os::raw::c_void) -> *const c_char;
    fn vardecl_get_type_inst(vardecl_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;
    fn vardecl_get_expression(vardecl_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for VarDeclI toplevel
    fn vardecl_is_toplevel(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for VarDeclI introduced
    fn vardecl_is_introduced(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for VarDeclI evaluated
    fn vardecl_is_evaluated(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for VarDeclI payload
    fn vardecl_get_payload(vardecl_ptr: *mut std::os::raw::c_void) -> i32;

    // New function for VarDeclI type alias
    fn vardecl_is_type_alias(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New functions for TypeInst inspection
    fn typeinst_get_base_type(typeinst_ptr: *mut std::os::raw::c_void) -> i32;

    // New function for TypeInst is_var
    fn typeinst_is_var(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_par
    fn typeinst_is_par(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_opt
    fn typeinst_is_opt(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_present
    fn typeinst_is_present(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_set
    fn typeinst_is_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int
    fn typeinst_is_int(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bool
    fn typeinst_is_bool(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_float
    fn typeinst_is_float(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_string
    fn typeinst_is_string(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_ann
    fn typeinst_is_ann(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_unknown
    fn typeinst_is_unknown(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_plain
    fn typeinst_is_plain(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bot
    fn typeinst_is_bot(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_top
    fn typeinst_is_top(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int_set
    fn typeinst_is_int_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bool_set
    fn typeinst_is_bool_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_float_set
    fn typeinst_is_float_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int_array
    fn typeinst_is_int_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int_set_array
    fn typeinst_is_int_set_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int_set_array
    fn typeinst_is_int_set_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bool_array
    fn typeinst_is_bool_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bool_array
    fn typeinst_is_bool_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New functions for Expression inspection
    fn expression_get_id(expr_ptr: *mut std::os::raw::c_void) -> i32;
    fn expression_is_intlit(expr_ptr: *mut std::os::raw::c_void) -> bool;

    // New functions for Expression float literal
    fn expression_is_floatlit(expr_ptr: *mut std::os::raw::c_void) -> bool;
    fn expression_as_floatlit(expr_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for FloatLit value
    fn floatlit_get_value(floatlit_ptr: *mut std::os::raw::c_void) -> f64;

// New functions for Expression set literal
bool expression_is_setlit(MiniZinc::Expression* expr_ptr);
MiniZinc::SetLit* expression_as_setlit(MiniZinc::Expression* expr_ptr);

// New functions for SetLit
unsigned int setlit_get_size(MiniZinc::SetLit* setlit_ptr);
MiniZinc::Expression* setlit_get_element_at_index(MiniZinc::SetLit* setlit_ptr, unsigned int index);

    // New functions for getting MiniZinc library paths
    fn minizinc_get_mznlib_dir(env_ptr: *mut MznSolver) -> *const c_char;
    fn minizinc_get_executable_path() -> *const c_char;

    // New function for MiniZincModel documentation comment
    fn minizinc_model_get_doc_comment(model_ptr: *mut std::os::raw::c_void) -> *const c_char;

    // New function for MiniZincModel parent
    fn minizinc_model_get_parent(model_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for MiniZincModel solve item
    fn minizinc_model_get_solve_item(model_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for MiniZincModel output item
    fn minizinc_model_get_output_item(model_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;
}

// Safe Rust wrappers for FFI functions
pub struct MiniZincEnvironment(pub *mut MznSolver);

impl MiniZincEnvironment {
    pub fn new() -> Result<Self, String> {
        let env_ptr = unsafe { minizinc_env_new() };
        if env_ptr.is_null() {
            Err("Failed to create MiniZinc environment".to_string())
        } else {
            Ok(MiniZincEnvironment(env_ptr))
        }
    }

    pub fn parse_model(&self, model_code: &str, filename: &str) -> Result<MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let model_ptr = unsafe {
            minizinc_parse_model(
                self.0,
                model_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if model_ptr.is_null() {
            Err("Failed to parse MiniZinc model".to_string())
        } else {
            Ok(MiniZincModel(model_ptr))
        }
    }

    pub fn parse_data(&self, model: &MiniZincModel, data_code: &str, filename: &str) -> Result<(), String> {
        let data_cstr = CString::new(data_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let result = unsafe {
            minizinc_parse_data_from_string(
                self.0,
                model.0,
                data_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if result != 0 {
            Err("Failed to parse MiniZinc data".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_version_string(&self) -> String {
        let version_cstr = unsafe { minizinc_get_version_string() };
        unsafe { CStr::from_ptr(version_cstr).to_str().unwrap().to_string() }
    }

    pub fn get_mznlib_dir(&self) -> String {
        let mznlib_dir_cstr = unsafe { minizinc_get_mznlib_dir(self.0) };
        unsafe { CStr::from_ptr(mznlib_dir_cstr).to_str().unwrap().to_string() }
    }

    pub fn get_executable_path() -> String {
        let path_cstr = unsafe { minizinc_get_executable_path() };
        unsafe { CStr::from_ptr(path_cstr).to_str().unwrap().to_string() }
    }
}

impl Drop for MiniZincEnvironment {
    fn drop(&mut self) {
        unsafe { minizinc_env_free(self.0) };
    }
}

impl Drop for MiniZincModel {
    fn drop(&mut self) {
        unsafe { minizinc_model_free(self.0) };
    }
}

impl MiniZincModel {
    pub fn filename(&self) -> String {
        let c_str = unsafe { model_get_filename(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn filepath(&self) -> String {
        let c_str = unsafe { model_get_filepath(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn num_items(&self) -> u32 {
        unsafe { model_get_num_items(self.0) }
    }

    pub fn get_item_at_index(&self, index: u32) -> Option<MiniZincItem> {
        let item_ptr = unsafe { model_get_item_at_index(self.0, index) };
        if item_ptr.is_null() {
            None
        } else {
            Some(MiniZincItem(item_ptr))
        }
    }

    pub fn doc_comment(&self) -> String {
        let c_str = unsafe { minizinc_model_get_doc_comment(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn parent(&self) -> Option<MiniZincModel> {
        let parent_ptr = unsafe { minizinc_model_get_parent(self.0) };
        if parent_ptr.is_null() {
            None
        } else {
            Some(MiniZincModel(parent_ptr))
        }
    }

    pub fn solve_item(&self) -> Option<MiniZincSolveItem> {
        let solve_item_ptr = unsafe { minizinc_model_get_solve_item(self.0) };
        if solve_item_ptr.is_null() {
            None
        } else {
            Some(MiniZincSolveItem(solve_item_ptr))
        }
    }

    pub fn output_item(&self) -> Option<MiniZincOutputItem> {
        let output_item_ptr = unsafe { minizinc_model_get_output_item(self.0) };
        if output_item_ptr.is_null() {
            None
        } else {
            Some(MiniZincOutputItem(output_item_ptr))
        }
    }
}

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

pub struct MiniZincVarDecl(pub *mut std::os::raw::c_void);

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

#[derive(Debug)]
pub struct MiniZincTypeInst(pub *mut std::os::raw::c_void);

impl MiniZincTypeInst {
    pub fn base_type(&self) -> MiniZincBaseType {
        let id = unsafe { typeinst_get_base_type(self.0) };
        id.into()
    }

    pub fn is_var(&self) -> bool {
        unsafe { typeinst_is_var(self.0) }
    }

    pub fn is_par(&self) -> bool {
        unsafe { typeinst_is_par(self.0) }
    }

    pub fn is_opt(&self) -> bool {
        unsafe { typeinst_is_opt(self.0) }
    }

    pub fn is_present(&self) -> bool {
        unsafe { typeinst_is_present(self.0) }
    }

    pub fn is_set(&self) -> bool {
        unsafe { typeinst_is_set(self.0) }
    }

    pub fn is_int(&self) -> bool {
        unsafe { typeinst_is_int(self.0) }
    }

    pub fn is_bool(&self) -> bool {
        unsafe { typeinst_is_bool(self.0) }
    }

    pub fn is_float(&self) -> bool {
        unsafe { typeinst_is_float(self.0) }
    }

    pub fn is_string(&self) -> bool {
        unsafe { typeinst_is_string(self.0) }
    }

    pub fn is_ann(&self) -> bool {
        unsafe { typeinst_is_ann(self.0) }
    }

    pub fn is_unknown(&self) -> bool {
        unsafe { typeinst_is_unknown(self.0) }
    }

    pub fn is_plain(&self) -> bool {
        unsafe { typeinst_is_plain(self.0) }
    }

    pub fn is_bot(&self) -> bool {
        unsafe { typeinst_is_bot(self.0) }
    }

    pub fn is_top(&self) -> bool {
        unsafe { typeinst_is_top(self.0) }
    }

    pub fn is_int_set(&self) -> bool {
        unsafe { typeinst_is_int_set(self.0) }
    }

    pub fn is_bool_set(&self) -> bool {
        unsafe { typeinst_is_bool_set(self.0) }
    }

    pub fn is_float_set(&self) -> bool {
        unsafe { typeinst_is_float_set(self.0) }
    }

    pub fn is_int_array(&self) -> bool {
        unsafe { typeinst_is_int_array(self.0) }
    }

    pub fn is_bool_array(&self) -> bool {
        unsafe { typeinst_is_bool_array(self.0) }
    }

    pub fn is_int_set_array(&self) -> bool {
        unsafe { typeinst_is_int_set_array(self.0) }
    }

    pub fn is_int_set_array(&self) -> bool {
        unsafe { typeinst_is_int_set_array(self.0) }
    }

    pub fn is_int_set_array(&self) -> bool {
        unsafe { typeinst_is_int_set_array(self.0) }
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_string() {
        let env = MiniZincEnvironment::new().unwrap();
        let version = env.get_version_string();
        println!("MiniZinc Version: {}", version);
        assert_eq!(version, "2.9.4-introspector");
    }

    #[test]
    fn test_env_creation_and_free() {
        let env = MiniZincEnvironment::new();
        assert!(env.is_ok());
        // Drop will be called automatically when env goes out of scope
    }
}
