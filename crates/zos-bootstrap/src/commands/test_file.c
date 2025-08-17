#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "minizinc_ffi_declarations_v2.h"
#include "minizinc_opaque_types.h"

int main() {
    printf(\"C ABI Test: Starting\n");

    MiniZincEnvWrapper* env_wrapper = minizinc_env_new();
    if (env_wrapper == NULL) {{
        fprintf(stderr, \"C ABI Test: Failed to create MiniZinc environment\n");
        return 1;
    }}
    printf(\"C ABI Test: MiniZinc environment created at %p\n\", (void*)env_wrapper);

    const char* model_str = \"var int: x; constraint x > 0; solve satisfy;\";
    MiniZincModel* model = minizinc_parse_string_only(env_wrapper, model_str);
    if (model == NULL) {{
        fprintf(stderr, \"C ABI Test: Failed to parse model\n");
        minizinc_env_free(env_wrapper);
        return 1;
    }}
    printf(\"C ABI Test: Model parsed successfully at %p\n\", (void*)model);

    minizinc_model_free(model);
    printf(\"C ABI Test: Model freed.\n");
    minizinc_env_free(env_wrapper);
    printf(\"C ABI Test: MiniZinc environment freed.\n");

    printf(\"C ABI Test: All C ABI tests passed successfully.\n");
    return 0;
}
