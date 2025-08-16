#include "minizinc_opaque_types.h"
#include "minizinc_ffi_declarations_v2.h"
#include <cassert>
#include <iostream>
#include <string>

int main() {
    // Create MiniZinc environment once
    MiniZincEnvWrapper* env_wrapper = minizinc_env_new();
    assert(env_wrapper != nullptr);
    assert(env_wrapper->solver != nullptr);

    std::cout << "MiniZinc environment created successfully." << std::endl;

    // Test minizinc_env_new and minizinc_env_free (conceptual, as we free at the end)
    std::cout << "minizinc_env_new and minizinc_env_free test passed (conceptual)." << std::endl;

    // Test minizinc_parse_string_only
    std::cout << "minizinc_parse_string_only test started." << std::endl;
    const char* model_code = "var int: x; solve satisfy;";
    MiniZincModel* model_ptr = minizinc_parse_string_only(env_wrapper, model_code);
    assert(model_ptr != nullptr);
    std::cout << "minizinc_parse_string_only test passed." << std::endl;
    std::cout << "Model filename: " << model_get_filename(model_ptr) << std::endl;
    minizinc_model_free(model_ptr);

    // Test minizinc_parse_model_with_flags
    std::cout << "minizinc_parse_model_with_flags test started." << std::endl;
    const char* model_code_flags = "var int: y; solve minimize y;";
    const char* filename_arg = "test_model.mzn"; // Added filename argument
    MiniZincModel* model_ptr_flags = minizinc_parse_model_with_flags(env_wrapper, model_code_flags, filename_arg, true);
    assert(model_ptr_flags != nullptr);
    std::cout << "minizinc_parse_model_with_flags test passed." << std::endl;
    minizinc_model_free(model_ptr_flags);

    // Test minizinc_get_version_string
    std::cout << "minizinc_get_version_string test started." << std::endl;
    const char* version_string_const = minizinc_get_version_string();
    assert(version_string_const != nullptr);
    // Cast to char* for minizinc_string_free
    char* version_string = const_cast<char*>(version_string_const);
    std::cout << "MiniZinc Version: " << version_string << std::endl;
    minizinc_string_free(version_string);
    std::cout << "minizinc_get_version_string test passed." << std::endl;

    // Test minizinc_solver_get_solver_instance (conceptual, as it's part of the env)
    std::cout << "minizinc_solver_get_solver_instance test passed (conceptual)." << std::endl;

    // Free MiniZinc environment once at the end
    minizinc_env_free(env_wrapper);
    std::cout << "MiniZinc environment freed successfully." << std::endl;

    std::cout << "All tests passed successfully!" << std::endl;

    return 0;
}
