use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Opaque types for MiniZincModel and Item
pub struct MiniZincModel(pub *mut std::os::raw::c_void);
pub struct MiniZincItem(pub *mut std::os::raw::c_void);

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
    fn minizinc_env_new() -> *mut std::os::raw::c_void;
    fn minizinc_env_free(env: *mut std::os::raw::c_void);
    fn minizinc_parse_model(
        env: *mut std::os::raw::c_void,
        model_str: *const c_char,
        filename: *const c_char,
    ) -> *mut std::os::raw::c_void;
    fn minizinc_parse_data_from_string(
        env: *mut std::os::raw::c_void,
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

    // New functions for VarDeclI inspection
    fn vardecl_get_id(vardecl_ptr: *mut std::os::raw::c_void) -> *const c_char;
    fn vardecl_get_type_inst(vardecl_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;
    fn vardecl_get_expression(vardecl_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for TypeInst inspection
    fn typeinst_get_base_type(typeinst_ptr: *mut std::os::raw::c_void) -> i32;

    // New functions for Expression inspection
    fn expression_get_id(expr_ptr: *mut std::os::raw::c_void) -> i32;
    fn expression_is_intlit(expr_ptr: *mut std::os::raw::c_void) -> bool;
}

// Safe Rust wrappers for FFI functions
pub struct MiniZincEnvironment(pub *mut std::os::raw::c_void);

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
                self.0, // Pass Flattener*
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
}

#[derive(Debug)]
pub struct MiniZincTypeInst(pub *mut std::os::raw::c_void);

impl MiniZincTypeInst {
    pub fn base_type(&self) -> MiniZincBaseType {
        let id = unsafe { typeinst_get_base_type(self.0) };
        id.into()
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

    #[test]
    fn test_parse_and_inspect_models() {
        let models = vec![
            ("var int: x = 1; solve satisfy;", "model_1.mzn"),
            ("var bool: b; constraint b; solve satisfy;", "model_2.mzn"),
            ("var float: f = 3.14; solve satisfy;", "model_3.mzn"),
        ];

        let env = MiniZincEnvironment::new().unwrap();

        for (model_code, filename) in models {
            println!("\n--- Parsing and Inspecting Model: {} ---", filename);
            let model = env.parse_model(model_code, filename).expect("Failed to parse MiniZinc model");

            println!("Parsed Model Filename: {}", model.filename());
            println!("Parsed Model Filepath: {}", model.filepath());
            println!("Parsed Model Num Items: {}", model.num_items());

            assert!(!model.filename().is_empty());
            assert!(!model.filepath().is_empty());
            assert!(model.num_items() > 0);

            for i in 0..model.num_items() {
                if let Some(item) = model.get_item_at_index(i) {
                    println!("  Item {}: ID: {:?}", i, item.item_id());
                    if item.is_vardecl() {
                        if let Some(vardecl) = item.as_vardecl() {
                            println!("    VarDecl ID: {}", vardecl.id());
                            let type_inst = vardecl.type_inst();
                            println!("    VarDecl TypeInst Base Type: {:?}", type_inst.base_type());
                            if let Some(expr) = vardecl.expression() {
                                println!("    VarDecl Expression ID: {:?}", expr.expression_id());
                                if expr.is_intlit() {
                                    println!("      Expression is IntLit!");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_data_from_string() {
        let env = MiniZincEnvironment::new().unwrap();
        // Model with x as a parameter (to be defined by data)
        let model_code = "int: x; solve satisfy;";
        let model_filename = "test_model_for_data.mzn";
        let model = env.parse_model(model_code, model_filename);
        assert!(model.is_ok()); // Ensure model parsing itself is successful
        let model = model.unwrap();

        let data_code = "x = 10;";
        let data_filename = "test_data.dzn";
        let result = env.parse_data(&model, data_code, data_filename); // Pass the model reference
        assert!(result.is_ok());

        // Model is automatically freed by Drop trait
    }
}
