#include "minizinc_c_wrapper.h"
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
#include <minizinc/flattener.hh>
#include <minizinc/solver.hh>

#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>

// Forward declarations for functions now in separate files
extern "C" {
Flattener* minizinc_env_new();
void minizinc_env_free(Flattener* env);
MiniZincModel* minizinc_parse_model(Flattener* env_ptr, const char* model_str, const char* filename);
int minizinc_parse_data_from_string(Flattener* env_ptr, MiniZincModel* model_ptr, const char* data_str, const char* filename);
void minizinc_model_free(MiniZincModel* model);
const char* minizinc_get_version_string();
}