#ifndef MINIZINC_C_WRAPPER_H
#define MINIZINC_C_WRAPPER_H

// Include MiniZinc headers directly here (OUTSIDE extern "C")
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
// #include <minizinc/version.hh>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations for opaque types
typedef void MiniZincModel;
typedef void MiniZincEnv;

// Function to create a new MiniZinc environment
MiniZincEnv* minizinc_env_new();

// Function to free a MiniZinc environment
void minizinc_env_free(MiniZincEnv* env);

// Function to parse a MiniZinc model from a string
MiniZincModel* minizinc_parse_model_from_string(MiniZincEnv* env, const char* model_str, const char* filename);

// Function to parse DZN data into a MiniZinc model
int minizinc_parse_data_from_string(MiniZincEnv* env, MiniZincModel* model, const char* data_str, const char* filename);

// Function to free a MiniZinc model
void minizinc_model_free(MiniZincModel* model);

// Function to get version string (for testing FFI)
const char* minizinc_get_version_string();

#ifdef __cplusplus
}
#endif

#endif // MINIZINC_C_WRAPPER_H