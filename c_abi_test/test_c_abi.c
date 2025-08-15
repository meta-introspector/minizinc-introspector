#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include the generated FFI header
#include "minizinc_ffi_declarations_v2.h"
#include "minizinc_opaque_types.h" // For MiniZincEnvWrapper

int main() {
    printf("C ABI Test: Starting\n");

    // 1. Create a new MiniZinc environment
    MiniZincEnvWrapper* env_wrapper = minizinc_env_new();
    if (env_wrapper == NULL) {
        fprintf(stderr, "C ABI Test: Failed to create MiniZinc environment\n");
        return 1;
    }
    printf("C ABI Test: MiniZinc environment created at %p\n", (void*)env_wrapper);

    // 2. Parse a simple model
    const char* model_str = "var int: x; constraint x > 0; solve satisfy;";
    MiniZincModel* model = minizinc_parse_string_only(env_wrapper, model_str);
    if (model == NULL) {
        fprintf(stderr, "C ABI Test: Failed to parse model\n");
        minizinc_env_free(env_wrapper);
        return 1;
    }
    printf("C ABI Test: Model parsed successfully at %p\n", (void*)model);

    // 3. Free the model and environment
    minizinc_model_free(model);
    printf("C ABI Test: Model freed.\n");
    minizinc_env_free(env_wrapper);
    printf("C ABI Test: MiniZinc environment freed.\n");

    printf("C ABI Test: All C ABI tests passed successfully.\n");
    return 0;
}
