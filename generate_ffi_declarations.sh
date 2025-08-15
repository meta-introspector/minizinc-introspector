#!/bin/bash

DECLARATIONS_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper_refactored/declarations"
OUTPUT_FILE="/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper_refactored/minizinc_ffi_declarations_v2.h"

echo "#!/bin/bash"

echo "#ifndef MINIZINC_FFI_DECLARATIONS_H" > "$OUTPUT_FILE"
echo "#define MINIZINC_FFI_DECLARATIONS_H" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "#include <minizinc/model.hh>" >> "$OUTPUT_FILE"
echo "#include <minizinc/parser.hh>" >> "$OUTPUT_FILE"
echo "#include <minizinc/ast.hh> // Added for ItemId" >> "$OUTPUT_FILE"
echo "#include <minizinc/solver.hh> // Include MznSolver" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "#include \"minizinc_opaque_types.h\"" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "#ifdef __cplusplus" >> "$OUTPUT_FILE"
echo "extern \"C\" {" >> "$OUTPUT_FILE"
echo "#endif" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "// Automatically generated FFI declarations" >> "$OUTPUT_FILE"

# Loop through all .h files in the declarations directory and add include statements
for file in "$DECLARATIONS_DIR"/*.h; do
    filename=$(basename "$file")
    echo "#include \"declarations/$filename\"" >> "$OUTPUT_FILE"
done

echo "" >> "$OUTPUT_FILE"
echo "#ifdef __cplusplus" >> "$OUTPUT_FILE"
echo "} // extern \"C\"" >> "$OUTPUT_FILE"
echo "#endif" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "#endif // MINIZINC_FFI_DECLARATIONS_H" >> "$OUTPUT_FILE"

echo "Generated $OUTPUT_FILE"
