#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh>
#include <minizinc/model.hh>
#include <minizinc/flattener.hh>
#include <minizinc/exception.hh>
#include <minizinc/solver_instance_base.hh>
#include <minizinc/solver_config.hh>
#include <minizinc/statistics.hh>
#include <minizinc/solns2out.hh>
#include <minizinc/timer.hh>
#include <minizinc/file_utils.hh>

#include <vector>
#include <string>
#include <iostream>

extern "C" {

int minizinc_solver_run(MiniZinc::MznSolver* solver_ptr, const char* model_str, const char** args, int num_args) {
    MiniZinc::MznSolver* solver = reinterpret_cast<MiniZinc::MznSolver*>(solver_ptr);
    std::string model_string = model_str ? model_str : "";

    std::vector<std::string> args_vec;
    for (int i = 0; i < num_args; ++i) {
        args_vec.push_back(args[i]);
    }

    // The run method takes an executable name, which we can set to "minizinc" or similar.
    // The model name can be "<string>" for string input.
    MiniZinc::SolverInstance::Status status = solver->run(args_vec, model_string, "minizinc", "<string>");

    return static_cast<int>(status);
}

} // extern "C"
