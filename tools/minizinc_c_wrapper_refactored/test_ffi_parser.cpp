#include "minizinc_ffi_declarations_v2.h"
#include <iostream>
#include <cassert>

int main() {
    // Test minizinc_env_new and minizinc_env_free
    MiniZincEnvWrapper* env_wrapper = minizinc_env_new();
    assert(env_wrapper != nullptr);
    assert(env_wrapper->solver != nullptr);
    assert(env_wrapper->timer != nullptr);

    std::cout << "minizinc_env_new and minizinc_env_free test passed." << std::endl;

    // Test minizinc_parse_string_only
    const char* model_str = "var int: x; solve satisfy;";
    MiniZincModel* model = minizinc_parse_string_only(env_wrapper, model_str);
    assert(model != nullptr);
    std::cout << "minizinc_parse_string_only test passed." << std::endl;

    // Test model_get_filename
    const char* filename = model_get_filename(model);
    std::cout << "Model filename: " << filename << std::endl;
    assert(std::string(filename) == "<string>");

    // Test model_get_filepath
    char* filepath = model_get_filepath(model);
    std::cout << "Model filepath: " << filepath << std::endl;
    assert(std::string(filepath) == "");
    minizinc_string_free(filepath);

    // Test model_get_num_items
    uint32_t num_items = model_get_num_items(model);
    std::cout << "Model num_items: " << num_items << std::endl;
    assert(num_items > 0);

    // Test minizinc_model_free
    minizinc_model_free(model);
    std::cout << "minizinc_model_free test passed." << std::endl;

    // Test minizinc_get_version_string
    const char* version_string = minizinc_get_version_string();
    std::cout << "MiniZinc Version: " << version_string << std::endl;
    assert(version_string != nullptr);

    // Test minizinc_solver_get_solver_instance (after a run)
    // Note: MznSolver::run needs a model to be loaded into the flattener
    // This test is more complex and requires a full solve cycle.
    // For now, we just ensure we can get the solver instance.
    MiniZinc::SolverInstanceBase* solver_instance_ptr = minizinc_solver_get_solver_instance(env_wrapper);
    assert(solver_instance_ptr != nullptr);
    std::cout << "minizinc_solver_get_solver_instance test passed." << std::endl;

    // Test minizinc_env_free
    minizinc_env_free(env_wrapper);
    std::cout << "All tests passed successfully!" << std::endl;

    return 0;
}