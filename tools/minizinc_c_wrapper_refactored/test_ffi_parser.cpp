#include <iostream>
#include <string>
#include "minizinc_ffi_declarations_v2.h"

int main() {
    std::cout << "[C++ Test] Starting FFI parser test." << std::endl;

    // 1. Create MiniZinc environment
    MiniZinc::MznSolver* env = minizinc_env_new();
    if (env == nullptr) {
        std::cerr << "[C++ Test] ERROR: Failed to create MiniZinc environment." << std::endl;
        return 1;
    }
    std::cout << "[C++ Test] MiniZinc environment created successfully." << std::endl;

    // 2. Define a simple MiniZinc model string
    const char* model_str = "var int: x; solve satisfy;";
    std::cout << "[C++ Test] Attempting to parse model: \"" << model_str << "\"" << std::endl;

    // 3. Call minizinc_parse_string_only()
    MiniZincModel* model = minizinc_parse_string_only(env, model_str);

    // 4. Check if the returned MiniZincModel* is null
    if (model == nullptr) {
        std::cerr << "[C++ Test] ERROR: minizinc_parse_string_only returned nullptr." << std::endl;
        minizinc_env_free(env);
        return 1;
    } else {
        std::cout << "[C++ Test] minizinc_parse_string_only returned non-null model pointer: " << model << std::endl;
    }

    // 5. Call minizinc_model_free() to deallocate the model
    std::cout << "[C++ Test] Attempting to free model..." << std::endl;
    minizinc_model_free(model);
    std::cout << "[C++ Test] Model freed successfully." << std::endl;

    // 6. Free the MiniZinc environment
    std::cout << "[C++ Test] Freeing MiniZinc environment..." << std::endl;
    minizinc_env_free(env);
    std::cout << "[C++ Test] MiniZinc environment freed successfully." << std::endl;

    std::cout << "[C++ Test] FFI parser test completed successfully." << std::endl;
    return 0;
}
