#ifndef MINIZINC_FFI_DECLARATIONS_H
#define MINIZINC_FFI_DECLARATIONS_H

#include <minizinc/flattener.hh>
#include <minizinc/model.hh>
#include <minizinc/parser.hh>

#include "minizinc_opaque_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Function to create a new MiniZinc environment
MiniZinc::Flattener* minizinc_env_new();

// Function to free a MiniZinc environment
void minizinc_env_free(MiniZinc::Flattener* env);

// Function to parse a MiniZinc model from a string
MiniZincModel* minizinc_parse_model(MiniZinc::Flattener* env, const char* model_str, const char* filename);

// Function to parse DZN data into a MiniZinc model
int minizinc_parse_data_from_string(MiniZinc::Flattener* env, MiniZincModel* model, const char* data_str, const char* filename);

// Function to free a MiniZinc model
void minizinc_model_free(MiniZincModel* model);

// Function to get version string (for testing FFI)
const char* minizinc_get_version_string();

// New functions for MiniZincModel inspection
const char* model_get_filename(MiniZincModel* model_ptr);
const char* model_get_filepath(MiniZincModel* model_ptr);
uint32_t model_get_num_items(MiniZincModel* model_ptr);
Item* model_get_item_at_index(MiniZincModel* model_ptr, uint32_t index);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // MINIZINC_FFI_DECLARATIONS_H