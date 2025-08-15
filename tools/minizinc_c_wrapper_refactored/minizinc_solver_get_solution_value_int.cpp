#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh>
#include <minizinc/solns2out.hh>
#include <string>
#include <regex>

extern "C" {

int minizinc_solver_get_solution_value_int(MiniZincEnvWrapper* wrapper_ptr, const char* var_name) {
    MiniZinc::MznSolver* solver = reinterpret_cast<MiniZinc::MznSolver*>(wrapper_ptr->solver);

    // Ensure the solution is evaluated and available in s2out.solution
    solver->s2out.evalOutput();

    std::string solution_str = solver->s2out.solution;
    std::string var_name_str = var_name;

    // Construct a regex to find the variable and its integer value
    // Example: "x = 6;"
    std::regex re(var_name_str + " = (\\d+);\n");
    std::smatch match;

    if (std::regex_search(solution_str, match, re) && match.size() > 1) {
        return std::stoi(match[1].str());
    } else {
        // Variable not found or not an integer
        return -1; // Indicate error or not found
    }
}

} // extern "C"