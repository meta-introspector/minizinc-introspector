use std::os::raw::c_char;
use crate::types::MznSolver;

unsafe extern "C" {
    pub fn minizinc_env_new() -> *mut MznSolver;
    pub fn minizinc_env_free(env: *mut MznSolver);
    pub fn minizinc_parse_model(
        env: *mut MznSolver,
        model_str: *const c_char,
        filename: *const c_char,
    ) -> *mut std::os::raw::c_void;
    pub fn minizinc_parse_data_from_string(
        env: *mut MznSolver,
        model: *mut std::os::raw::c_void,
        data_str: *const c_char,
        filename: *const c_char,
    ) -> std::os::raw::c_int;
    pub fn minizinc_model_free(model: *mut std::os::raw::c_void);
    pub fn minizinc_get_version_string() -> *const c_char;

    // New functions for MiniZincModel inspection
    pub fn model_get_filename(model_ptr: *mut std::os::raw::c_void) -> *const c_char;
    pub fn model_get_filepath(model_ptr: *mut std::os::raw::c_void) -> *const c_char;
    pub fn model_get_num_items(model_ptr: *mut std::os::raw::c_void) -> u32;
    pub fn model_get_item_at_index(model_ptr: *mut std::os::raw::c_void, index: u32) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem inspection
    pub fn item_get_id(item_ptr: *mut std::os::raw::c_void) -> i32;
    pub fn item_is_vardecl(item_ptr: *mut std::os::raw::c_void) -> bool;
    pub fn item_as_vardecl(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem assignment
    pub fn item_is_assign(item_ptr: *mut std::os::raw::c_void) -> bool;
    pub fn item_as_assign(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem constraint
    pub fn item_is_constraint(item_ptr: *mut std::os::raw::c_void) -> bool;
    pub fn item_as_constraint(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem include
    pub fn item_is_include(item_ptr: *mut std::os::raw::c_void) -> bool;
    pub fn item_as_include(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for MiniZincItem function
    pub fn item_is_function(item_ptr: *mut std::os::raw::c_void) -> bool;
    pub fn item_as_function(item_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for VarDeclI inspection
    pub fn vardecl_get_id(vardecl_ptr: *mut std::os::raw::c_void) -> *const c_char;
    pub fn vardecl_get_type_inst(vardecl_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;
    pub fn vardecl_get_expression(vardecl_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for VarDeclI toplevel
    pub fn vardecl_is_toplevel(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for VarDeclI introduced
    pub fn vardecl_is_introduced(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for VarDeclI evaluated
    pub fn vardecl_is_evaluated(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for VarDeclI payload
    pub fn vardecl_get_payload(vardecl_ptr: *mut std::os::raw::c_void) -> i32;

    // New function for VarDeclI type alias
    pub fn vardecl_is_type_alias(vardecl_ptr: *mut std::os::raw::c_void) -> bool;

    // New functions for TypeInst inspection
    pub fn typeinst_get_base_type(typeinst_ptr: *mut std::os::raw::c_void) -> i32;

    // New function for TypeInst is_var
    pub fn typeinst_is_var(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_par
    pub fn typeinst_is_par(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_opt
    pub fn typeinst_is_opt(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_present
    pub fn typeinst_is_present(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_set
    pub fn typeinst_is_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int
    pub fn typeinst_is_int(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bool
    pub fn typeinst_is_bool(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_float
    pub fn typeinst_is_float(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_string
    pub fn typeinst_is_string(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_ann
    pub fn typeinst_is_ann(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_unknown
    pub fn typeinst_is_unknown(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_plain
    pub fn typeinst_is_plain(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bot
    pub fn typeinst_is_bot(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_top
    pub fn typeinst_is_top(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int_set
    pub fn typeinst_is_int_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bool_set
    pub fn typeinst_is_bool_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_float_set
    pub fn typeinst_is_float_set(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int_array
    pub fn typeinst_is_int_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_bool_array
    pub fn typeinst_is_bool_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New function for TypeInst is_int_set_array
    pub fn typeinst_is_int_set_array(typeinst_ptr: *mut std::os::raw::c_void) -> bool;

    // New functions for Expression inspection
    pub fn expression_get_id(expr_ptr: *mut std::os::raw::c_void) -> i32;
    pub fn expression_is_intlit(expr_ptr: *mut std::os::raw::c_void) -> bool;

    // New functions for Expression float literal
    pub fn expression_is_floatlit(expr_ptr: *mut std::os::raw::c_void) -> bool;
    pub fn expression_as_floatlit(expr_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for FloatLit value
    pub fn floatlit_get_value(floatlit_ptr: *mut std::os::raw::c_void) -> f64;

    // New functions for Expression set literal
    pub fn expression_is_setlit(expr_ptr: *mut std::os::raw::c_void) -> bool;
    pub fn expression_as_setlit(expr_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New functions for SetLit
    pub fn setlit_get_size(setlit_ptr: *mut std::os::raw::c_void) -> u32;
    pub fn setlit_get_element_at_index(setlit_ptr: *mut std::os::raw::c_void, index: u32) -> *mut std::os::raw::c_void;

    // New functions for getting MiniZinc library paths
    pub fn minizinc_get_mznlib_dir(env_ptr: *mut MznSolver) -> *const c_char;
    pub fn minizinc_get_executable_path() -> *const c_char;

    // New function for MiniZincModel documentation comment
    pub fn minizinc_model_get_doc_comment(model_ptr: *mut std::os::raw::c_void) -> *const c_char;

    // New function for MiniZincModel parent
    pub fn minizinc_model_get_parent(model_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for MiniZincModel solve item
    pub fn minizinc_model_get_solve_item(model_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

    // New function for MiniZincModel output item
    pub fn minizinc_model_get_output_item(model_ptr: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;
}
