#ifndef MINIZINC_FFI_DECLARATIONS_H
#define MINIZINC_FFI_DECLARATIONS_H

#include <minizinc/model.hh>
#include <minizinc/parser.hh>
#include <minizinc/ast.hh> // Added for ItemId
#include <minizinc/solver.hh> // Include MznSolver

#include "minizinc_opaque_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Function to create a new MiniZinc environment
MiniZinc::MznSolver* minizinc_env_new();

// Function to free a MiniZinc environment
void minizinc_env_free(MiniZinc::MznSolver* env);

// Function to parse a MiniZinc model from a string
MiniZincModel* minizinc_parse_model(MiniZinc::MznSolver* env, const char* model_str, const char* filename);

// Function to parse DZN data into a MiniZinc model
int minizinc_parse_data_from_string(MiniZinc::MznSolver* env, MiniZincModel* model, const char* data_str, const char* filename);

// Function to free a MiniZinc model
void minizinc_model_free(MiniZincModel* model);

// Function to get version string (for testing FFI)
const char* minizinc_get_version_string();

// New functions for MiniZincModel inspection
const char* model_get_filename(MiniZincModel* model_ptr);
const char* model_get_filepath(MiniZincModel* model_ptr);
uint32_t model_get_num_items(MiniZincModel* model_ptr);
Item* model_get_item_at_index(MiniZincModel* model_ptr, uint32_t index);

// New function for MiniZincModel documentation comment
const char* minizinc_model_get_doc_comment(MiniZincModel* model_ptr);

// New function for MiniZincModel parent
MiniZincModel* minizinc_model_get_parent(MiniZincModel* model_ptr);

// New function for MiniZincModel solve item
SolveI* minizinc_model_get_solve_item(MiniZincModel* model_ptr);

// New function for MiniZincModel output item
OutputI* minizinc_model_get_output_item(MiniZincModel* model_ptr);

// New functions for MiniZincItem inspection
int item_get_id(Item* item_ptr);
bool item_is_vardecl(Item* item_ptr);
MiniZinc::VarDeclI* item_as_vardecl(Item* item_ptr);

// New functions for MiniZincItem assignment
bool item_is_assign(Item* item_ptr);
MiniZinc::AssignI* item_as_assign(Item* item_ptr);

// New functions for MiniZincItem constraint
bool item_is_constraint(Item* item_ptr);
MiniZinc::ConstraintI* item_as_constraint(Item* item_ptr);

// New functions for MiniZincItem include
bool item_is_include(Item* item_ptr);
MiniZinc::IncludeI* item_as_include(Item* item_ptr);

// New functions for MiniZincItem function
bool item_is_function(Item* item_ptr);
MiniZinc::FunctionI* item_as_function(Item* item_ptr);

// New functions for VarDeclI inspection
const char* vardecl_get_id(MiniZinc::VarDeclI* vardecl_ptr);
MiniZinc::TypeInst* vardecl_get_type_inst(MiniZinc::VarDeclI* vardecl_ptr);
MiniZinc::Expression* vardecl_get_expression(MiniZinc::VarDeclI* vardecl_ptr);

// New function for VarDeclI toplevel
bool vardecl_is_toplevel(MiniZinc::VarDeclI* vardecl_ptr);

// New function for VarDeclI introduced
bool vardecl_is_introduced(MiniZinc::VarDeclI* vardecl_ptr);

// New function for VarDeclI evaluated
bool vardecl_is_evaluated(MiniZinc::VarDeclI* vardecl_ptr);

// New functions for TypeInst inspection
int typeinst_get_base_type(MiniZinc::TypeInst* typeinst_ptr);

// New functions for Expression inspection
int expression_get_id(MiniZinc::Expression* expr_ptr);
bool expression_is_intlit(MiniZinc::Expression* expr_ptr);

// New functions for getting MiniZinc library paths
const char* minizinc_get_mznlib_dir(MiniZinc::MznSolver* env_ptr);
const char* minizinc_get_executable_path();

#ifdef __cplusplus
} // extern "C"
#endif

#endif // MINIZINC_FFI_DECLARATIONS_H
