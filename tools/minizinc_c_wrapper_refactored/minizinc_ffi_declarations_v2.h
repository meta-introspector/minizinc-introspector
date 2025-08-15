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

// Automatically generated FFI declarations
#include "declarations/item_as_assign.h"
#include "declarations/item_as_constraint.h"
#include "declarations/item_as_function.h"
#include "declarations/item_as_include.h"
#include "declarations/item_as_vardecl.h"
#include "declarations/item_get_id.h"
#include "declarations/item_is_assign.h"
#include "declarations/item_is_constraint.h"
#include "declarations/item_is_function.h"
#include "declarations/item_is_include.h"
#include "declarations/item_is_vardecl.h"
#include "declarations/minizinc_env_free.h"
#include "declarations/minizinc_env_new.h"
#include "declarations/minizinc_get_executable_path.h"
#include "declarations/minizinc_get_version_string.h"
#include "declarations/minizinc_model_free.h"
#include "declarations/minizinc_model_get_doc_comment.h"
#include "declarations/minizinc_model_get_output_item.h"
#include "declarations/minizinc_model_get_parent.h"
#include "declarations/minizinc_model_get_solve_item.h"
#include "declarations/minizinc_parse_model.h"
#include "declarations/minizinc_parse_model_with_flags.h"
#include "declarations/minizinc_parse_string_only.h"
#include "declarations/minizinc_solver_free.h"
#include "declarations/minizinc_solver_get_solution_value_int.h"
#include "declarations/minizinc_solver_get_solver_instance.h"
#include "declarations/minizinc_solver_instance_next.h"
#include "declarations/minizinc_solver_instance_print_solution.h"
#include "declarations/minizinc_string_free.h"
#include "declarations/model_get_filename.h"
#include "declarations/model_get_filepath.h"
#include "declarations/model_get_item_at_index.h"
#include "declarations/model_get_num_items.h"
#include "declarations/typeinst_get_base_type.h"
#include "declarations/typeinst_is_ann.h"
#include "declarations/typeinst_is_bool.h"
#include "declarations/typeinst_is_bool_array.h"
#include "declarations/typeinst_is_bool_set.h"
#include "declarations/typeinst_is_bot.h"
#include "declarations/typeinst_is_float.h"
#include "declarations/typeinst_is_float_set.h"
#include "declarations/typeinst_is_int.h"
#include "declarations/typeinst_is_int_array.h"
#include "declarations/typeinst_is_int_set.h"
#include "declarations/typeinst_is_int_set_array.h"
#include "declarations/typeinst_is_opt.h"
#include "declarations/typeinst_is_par.h"
#include "declarations/typeinst_is_plain.h"
#include "declarations/typeinst_is_present.h"
#include "declarations/typeinst_is_set.h"
#include "declarations/typeinst_is_string.h"
#include "declarations/typeinst_is_top.h"
#include "declarations/typeinst_is_unknown.h"
#include "declarations/typeinst_is_var.h"
#include "declarations/vardecl_get_expression.h"
#include "declarations/vardecl_get_id.h"
#include "declarations/vardecl_get_payload.h"
#include "declarations/vardecl_get_type_inst.h"
#include "declarations/vardecl_is_evaluated.h"
#include "declarations/vardecl_is_introduced.h"
#include "declarations/vardecl_is_toplevel.h"
#include "declarations/vardecl_is_type_alias.h"

#ifdef __cplusplus
} // extern "C"
#endif

#endif // MINIZINC_FFI_DECLARATIONS_H
