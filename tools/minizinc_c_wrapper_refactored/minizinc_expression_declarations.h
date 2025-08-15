#ifndef MINIZINC_EXPRESSION_DECLARATIONS_H
#define MINIZINC_EXPRESSION_DECLARATIONS_H

#include <minizinc/ast.hh>
#include "minizinc_opaque_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// New functions for Expression inspection
int expression_get_id(MiniZinc::Expression* expr_ptr);
bool expression_is_intlit(MiniZinc::Expression* expr_ptr);

// New functions for Expression float literal
bool expression_is_floatlit(MiniZinc::Expression* expr_ptr);
MiniZinc::FloatLit* expression_as_floatlit(MiniZinc::Expression* expr_ptr);

// New function for FloatLit value
double floatlit_get_value(MiniZinc::FloatLit* floatlit_ptr);

// New functions for Expression set literal
bool expression_is_setlit(MiniZinc::Expression* expr_ptr);
MiniZinc::SetLit* expression_as_setlit(MiniZinc::Expression* expr_ptr);

// New functions for SetLit
unsigned int setlit_get_size(MiniZinc::SetLit* setlit_ptr);
MiniZinc::Expression* setlit_get_element_at_index(MiniZinc::SetLit* setlit_ptr, unsigned int index);

// New functions for Expression boolean literal
bool expression_is_boollit(MiniZinc::Expression* expr_ptr);
MiniZinc::BoolLit* expression_as_boollit(MiniZinc::Expression* expr_ptr);

// New function for BoolLit value
bool boollit_get_value(MiniZinc::BoolLit* boollit_ptr);

// New functions for Expression string literal
bool expression_is_stringlit(MiniZinc::Expression* expr_ptr);
MiniZinc::StringLit* expression_as_stringlit(MiniZinc::Expression* expr_ptr);

// New function for StringLit value
const char* stringlit_get_value(MiniZinc::StringLit* stringlit_ptr);

// New functions for Expression identifier
bool expression_is_id(MiniZinc::Expression* expr_ptr);
MiniZinc::Id* expression_as_id(MiniZinc::Expression* expr_ptr);

// New function for Id value
const char* id_get_value(MiniZinc::Id* id_ptr);

// New functions for Expression anonymous variable
bool expression_is_anon_var(MiniZinc::Expression* expr_ptr);
MiniZinc::AnonVar* expression_as_anon_var(MiniZinc::Expression* expr_ptr);

// New functions for Expression array literal
bool expression_is_arraylit(MiniZinc::Expression* expr_ptr);
MiniZinc::ArrayLit* expression_as_arraylit(MiniZinc::Expression* expr_ptr);

// New functions for ArrayLit
unsigned int arraylit_get_size(MiniZinc::ArrayLit* arraylit_ptr);
MiniZinc::Expression* arraylit_get_element_at_index(MiniZinc::ArrayLit* arraylit_ptr, unsigned int index);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // MINIZINC_EXPRESSION_DECLARATIONS_H
